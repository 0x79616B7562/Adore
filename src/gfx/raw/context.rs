#[allow(deprecated)]
use raw_window_handle::{
    HasRawDisplayHandle,
    HasRawWindowHandle,
};

use crate::gfx::raw::{
    ContextConfig,
    Depth,
    Frame,
};

// DO NOT TOUCH MY LOVELY SINGLETON!
pub(crate) static mut CONTEXT: Option<Context> = None;

#[allow(dead_code)]
pub fn init<T>(handle: &T, size: (u32, u32))
where T: raw_window_handle::HasWindowHandle + raw_window_handle::HasDisplayHandle {
    #[allow(deprecated)]
    unsafe {
        CONTEXT = Some(Context::new(
            handle.raw_display_handle().unwrap(),
            handle.raw_window_handle().unwrap(),
            size,
        ));
    }
}

#[allow(dead_code)]
#[inline]
pub fn reset(config: ContextConfig) {
    ctx!().reset(config);
}

#[allow(dead_code)]
#[inline]
pub fn render<T>(mut f: T)
where T: FnMut() {
    ctx!().render(|| {
        f();
    });
}

#[allow(dead_code)]
#[inline]
pub fn limits() -> wgpu::Limits {
    ctx!().device.limits()
}

#[allow(dead_code)]
#[inline]
pub fn features() -> wgpu::Features {
    ctx!().device.features()
}

#[allow(dead_code)]
#[inline]
pub fn device() -> &'static wgpu::Device {
    &ctx!().device
}

#[allow(dead_code)]
#[inline]
pub fn format() -> wgpu::TextureFormat {
    ctx!().config.format
}

#[allow(dead_code)]
#[inline]
pub fn queue() -> &'static wgpu::Queue {
    &ctx!().queue
}

#[allow(dead_code)]
#[inline]
pub fn frame() -> Option<&'static mut Frame> {
    ctx!().frame.as_mut()
}

//

pub(crate) struct Context {
    pub(crate) surface: wgpu::Surface<'static>,
    pub(crate) device: wgpu::Device,
    pub(crate) queue: wgpu::Queue,
    pub(crate) config: wgpu::SurfaceConfiguration,

    pub(crate) depth_texture: Depth,
    pub(crate) frame: Option<Frame>,
}

impl Context {
    #[allow(deprecated)]
    pub fn new(
        display_handle: raw_window_handle::RawDisplayHandle,
        window_handle: raw_window_handle::RawWindowHandle,
        size: (u32, u32),
    ) -> Self {
        pollster::block_on(Context::_new(display_handle, window_handle, size))
    }

    #[allow(deprecated)]
    async fn _new(
        display_handle: raw_window_handle::RawDisplayHandle,
        window_handle: raw_window_handle::RawWindowHandle,
        size: (u32, u32),
    ) -> Self {
        let backends = wgpu::Backends::all();

        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends,
            flags: wgpu::InstanceFlags::default(),
            dx12_shader_compiler: wgpu::Dx12Compiler::default(),
            gles_minor_version: wgpu::Gles3MinorVersion::default(),
        });

        let surface = unsafe {
            match instance.create_surface_unsafe(wgpu::SurfaceTargetUnsafe::RawHandle {
                raw_display_handle: display_handle,
                raw_window_handle: window_handle,
            }) {
                Ok(surface) => surface,
                Err(e) => {
                    log::error!("Error while creating surface: {:?}", e);
                    panic!();
                },
            }
        };

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();

        let features = wgpu::Features::empty();
        log::trace!("Requested features: {:?}", features);
        let features = Context::request_features(&adapter, features);

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    required_features: features,
                    required_limits: if cfg!(target_arch = "wasm32") {
                        wgpu::Limits::downlevel_webgl2_defaults()
                    } else {
                        adapter.limits()
                    },
                    label: None,
                },
                None,
            )
            .await
            .unwrap();

        log::trace!("Supported features: {:?}", features);

        let surface_caps = surface.get_capabilities(&adapter);

        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_caps.formats[0]);

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.0,
            height: size.1,
            present_mode: wgpu::PresentMode::AutoNoVsync,
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        surface.configure(&device, &config);

        log::trace!("Backend: {:?}", adapter.get_info().backend);

        let depth_texture = Depth::create_depth_texture(&device, &config, "depth");

        Context {
            surface,
            device,
            queue,
            config,

            depth_texture,
            frame: None,
        }
    }

    fn request_features(adapter: &wgpu::Adapter, features: wgpu::Features) -> wgpu::Features {
        let mut out = wgpu::Features::empty();

        for feature in features.iter() {
            if adapter.features().contains(feature) {
                out |= feature;
            }
        }

        out
    }

    pub fn reset(&mut self, config: ContextConfig) {
        if config.width > 0 && config.height > 0 {
            self.config.width = config.width;
            self.config.height = config.height;

            self.config.present_mode = if config.vsync {
                wgpu::PresentMode::AutoVsync
            } else {
                wgpu::PresentMode::AutoNoVsync
            };

            self.surface.configure(&self.device, &self.config);

            self.depth_texture = Depth::create_depth_texture(&self.device, &self.config, "depth");
        }
    }

    pub fn render<T>(&mut self, mut func: T)
    where T: FnMut() {
        match self.surface.get_current_texture() {
            Ok(output) => {
                let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());

                self.frame = Some(Frame::new(output, view, self.create_encoder()));

                func();

                let frame = self.frame.take().unwrap();
                self.queue.submit(std::iter::once(frame.encoder.finish()));
                frame.output.present();
            },
            Err(err) => {
                log::error!("{:?}", err);
            },
        }

        crate::gfx::raw::reset_dynamic_uniforms();
    }

    #[inline]
    pub fn create_encoder(&self) -> wgpu::CommandEncoder {
        self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Encoder"),
        })
    }
}

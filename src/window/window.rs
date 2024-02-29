use winit::{
    dpi::{
        PhysicalPosition,
        PhysicalSize,
    },
    event::{
        Event,
        WindowEvent,
    },
    event_loop::{
        ControlFlow,
        EventLoop,
    },
    window::{
        Window as WinitWindow,
        WindowBuilder,
    },
};

use crate::{
    types::Size,
    window::Input,
};

//

static mut EXIT: bool = false;

pub fn abort() {
    unsafe {
        EXIT = true;
    }
}

static mut INPUT: Option<Input> = None;

pub fn input_mut() -> &'static mut Input {
    unsafe { INPUT.as_mut().unwrap() }
}

pub fn input() -> &'static Input {
    unsafe { INPUT.as_ref().unwrap() }
}

static mut RAW: Option<WinitWindow> = None;

pub fn raw() -> &'static mut WinitWindow {
    unsafe { RAW.as_mut().unwrap() }
}

//

#[derive(Debug)]
pub struct Window {
    event_loop: EventLoop<()>,
    size: PhysicalSize<u32>,
}

unsafe impl Sync for Window {
}

unsafe impl Send for Window {
}

impl raw_window_handle::HasDisplayHandle for Window {
    fn display_handle(&self) -> Result<raw_window_handle::DisplayHandle<'_>, raw_window_handle::HandleError> {
        unsafe { RAW.as_ref().unwrap().display_handle() }
    }
}

impl raw_window_handle::HasWindowHandle for Window {
    fn window_handle(&self) -> Result<raw_window_handle::WindowHandle<'_>, raw_window_handle::HandleError> {
        unsafe { RAW.as_ref().unwrap().window_handle() }
    }
}

impl Window {
    pub fn new(title: &str, width: u32, height: u32, resizable: bool) -> Self {
        unsafe {
            INPUT = Some(Input::new());
        }

        let event_loop = EventLoop::new().unwrap();

        let window = WindowBuilder::new()
            .with_visible(false)
            .with_title(title)
            .with_inner_size(PhysicalSize::new(width, height))
            .with_resizable(resizable)
            .build(&event_loop)
            .unwrap();

        #[cfg(any(target_os = "windows", target_os = "linux", target_os = "macos"))]
        {
            if let Some(monitor) = window.current_monitor() {
                window.set_outer_position(PhysicalPosition::new(
                    monitor.size().width / 2 - window.inner_size().width / 2,
                    monitor.size().height / 2 - window.inner_size().height / 2,
                ));
            }
        }

        let size = window.inner_size();
        window.set_visible(true);

        unsafe {
            RAW = Some(window);
        }

        Self {
            event_loop,
            size,
        }
    }

    pub fn run<T>(mut self, mut func: T)
    where T: FnMut(Size<u32>) + 'static {
        self.event_loop.set_control_flow(ControlFlow::Poll);

        self.event_loop
            .run(move |event, elwt| match event {
                Event::WindowEvent {
                    event, ..
                } => match event {
                    WindowEvent::CloseRequested => elwt.exit(),
                    WindowEvent::Resized(size) => self.size = size,
                    WindowEvent::KeyboardInput {
                        event, ..
                    } => match event.state {
                        winit::event::ElementState::Pressed => input_mut().process_key(event.physical_key, true),
                        winit::event::ElementState::Released => input_mut().process_key(event.physical_key, false),
                    },
                    WindowEvent::CursorMoved {
                        position, ..
                    } => input_mut().set_mouse_position(position.into()),
                    WindowEvent::MouseInput {
                        state,
                        button,
                        ..
                    } => match state {
                        winit::event::ElementState::Pressed => input_mut().process_mouse_button(button, true),
                        winit::event::ElementState::Released => input_mut().process_mouse_button(button, false),
                    },
                    WindowEvent::RedrawRequested => {
                        unsafe {
                            if EXIT {
                                elwt.exit();
                            }
                        }

                        func(Size {
                            width: self.size.width,
                            height: self.size.height,
                        });

                        input_mut().reset();
                    },
                    _ => (),
                },
                Event::DeviceEvent {
                    event, ..
                } => match event {
                    winit::event::DeviceEvent::MouseMotion {
                        delta,
                    } => input_mut().process_mouse_motion(delta),
                    winit::event::DeviceEvent::MouseWheel {
                        delta,
                    } => {
                        let (x, y) = match delta {
                            winit::event::MouseScrollDelta::LineDelta(x, y) => (x, y),
                            winit::event::MouseScrollDelta::PixelDelta(p) => (p.x as f32, p.y as f32),
                        };

                        input_mut().process_mouse_wheel(x, y);
                    },
                    _ => (),
                },
                Event::AboutToWait => {
                    raw().request_redraw();
                },
                _ => (),
            })
            .unwrap();
    }

    pub fn size(&self) -> (u32, u32) {
        raw().inner_size().into()
    }
}

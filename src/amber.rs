use crate::{
    time::GameTime,
    traits::App,
    types::Size,
    window::{
        Window,
        WindowConfig,
    },
};

//

#[derive(Debug, Clone, Copy)]
pub struct AmberConfig {
    pub window_config: WindowConfig,
}

#[allow(clippy::all)]
impl Default for AmberConfig {
    fn default() -> Self {
        Self {
            window_config: WindowConfig::default(),
        }
    }
}

//

#[derive(Debug)]
pub struct Amber {
    window: Window,

    game_time: GameTime,
}

impl Amber {
    pub fn new(config: AmberConfig) -> Self {
        let window = Window::new(config.window_config);

        crate::gfx::raw::init(&window, window.size());

        Self {
            window,

            game_time: GameTime::new(),
        }
    }

    pub fn run(mut self, mut application: impl App + 'static) {
        let mut old_size = Size::default();

        self.window.run(move |size| {
            if size != old_size {
                old_size = size;

                crate::gfx::raw::reset(crate::gfx::raw::ContextConfig {
                    width: size.width,
                    height: size.height,
                    vsync: false,
                });

                application.resize(size);
            }

            application.update(self.game_time);

            crate::gfx::raw::render(|| {
                {
                    // dummy render pass
                    if let Some(frame) = crate::gfx::raw::frame() {
                        _ = frame.create_render_pass_with_load_op(false, crate::gfx::raw::LoadOp::Clear(crate::gfx::raw::Color::default()));
                    }
                }

                application.draw();
            });

            self.game_time.update();
        });
    }
}

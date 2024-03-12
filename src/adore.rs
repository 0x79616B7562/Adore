use crate::{
    time::GameTime,
    traits::Game,
    types::Size,
    window::{
        Window,
        WindowConfig,
    },
};

//

#[derive(Debug, Clone, Copy)]
pub struct AdoreConfig {
    pub window_config: WindowConfig,
}

#[allow(clippy::all)]
impl Default for AdoreConfig {
    fn default() -> Self {
        Self {
            window_config: WindowConfig::default(),
        }
    }
}

//

#[derive(Debug)]
pub struct Adore {
    window: Window,

    game_time: GameTime,
}

impl Adore {
    pub fn new(config: AdoreConfig) -> Self {
        let window = Window::new(config.window_config);

        crate::gfx::raw::init(&window, window.size());

        Self {
            window,

            game_time: GameTime::new(),
        }
    }

    pub fn run(mut self, mut game: impl Game + 'static) {
        let mut old_size = Size::default();

        self.window.run(move |size| {
            if size != old_size {
                old_size = size;

                crate::gfx::raw::reset(crate::gfx::raw::ContextConfig {
                    width: size.width,
                    height: size.height,
                    vsync: false,
                });

                game.resize(size);
            }

            game.update(self.game_time);

            crate::gfx::raw::render(|| {
                {
                    // dummy render pass
                    if let Some(frame) = crate::gfx::raw::frame() {
                        _ = frame.create_render_pass_with_load_op(false, crate::gfx::raw::LoadOp::Clear(crate::gfx::raw::Color::default()));
                    }
                }

                game.draw(self.game_time);
            });

            self.game_time.update();
        });
    }
}

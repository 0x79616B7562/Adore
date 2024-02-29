use crate::{
    time::GameTime,
    traits::App,
    types::Size,
    window::Window,
};

#[derive(Debug)]
pub struct Amber {
    window: Window,

    game_time: GameTime,
}

impl Default for Amber {
    fn default() -> Self {
        let window = Window::new("", 1280, 720, true);

        crate::gfx::raw::init(&window, window.size());

        Self {
            window,

            game_time: GameTime::new(),
        }
    }
}

impl Amber {
    pub fn new() -> Self {
        Self::default()
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

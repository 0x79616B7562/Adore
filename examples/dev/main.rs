use std::path::Path;

struct App {
    batch: adore::Batch,

    sprite: adore::Sprite,
    sprite0: adore::Sprite,

    fps: Vec<f32>,
    throttle: f32,
}

impl App {
    pub fn new() -> Self {
        let batch = adore::Batch::new();

        let sprite = adore::Sprite::new(adore::AssetManager::load_texture(Path::new("examples/dev/test.png")).unwrap());
        let sprite0 = adore::Sprite::new(adore::AssetManager::load_texture(Path::new("examples/dev/dev.png")).unwrap());

        Self {
            batch,

            sprite,
            sprite0,

            fps: vec![],
            throttle: 0.0,
        }
    }
}

impl adore::Game for App {
    fn resize(&mut self, _size: adore::Size<u32>) {
    }

    fn update(&mut self, game_time: adore::GameTime) {
        if adore::input().key_just_pressed(adore::KeyCode::Escape) {
            adore::abort();
        }

        if self.throttle > 0.0 {
            self.throttle -= game_time.delta();
            self.fps.push(1.0 / game_time.delta());
        } else {
            self.fps.push(1.0 / game_time.delta());
            self.throttle = 1.0;

            adore::log::trace!(
                "FPS: {:.2}, TOTAL: {:.2}",
                self.fps.iter().sum::<f32>() / self.fps.len() as f32,
                game_time.total(),
            );

            self.fps.clear();
        }
    }

    fn draw(&mut self) {
        self.batch.begin();

        // self.sprite0.target_mut().x = 100.0;
        // for y in 0..10 {
        //     self.sprite0.target_mut().y = y as f32 * self.sprite0.target().height;
        //     self.batch.draw_sprite(&self.sprite0);
        // }

        // self.batch.draw_sprite(&self.sprite);

        // self.batch.draw_sprite(&self.sprite0);
        // self.batch.draw_sprite(&self.sprite);

        // for x in 0..10 {
        //     self.sprite.target_mut().x = x as f32 * self.sprite.target().width / 2.;
        //     self.sprite.target_mut().y = x as f32 * self.sprite.target().height / 2.;
        //     self.sprite0.set_target(self.sprite.target());

        //     self.batch.draw_sprite(&self.sprite);
        // }

        for x in 0..32 {
            for y in 0..20 {
                if x % 2 == 1 {
                    if y % 2 == 0 {
                        self.sprite.target_mut().x = x as f32 * self.sprite.target().width;
                        self.sprite.target_mut().y = y as f32 * self.sprite.target().height;
                        self.batch.draw_sprite(&self.sprite);
                    } else {
                        self.sprite0.target_mut().x = x as f32 * self.sprite0.target().width;
                        self.sprite0.target_mut().y = y as f32 * self.sprite0.target().height;
                        self.batch.draw_sprite(&self.sprite0);
                    }
                } else {
                    if y % 2 == 0 {
                        self.sprite0.target_mut().x = x as f32 * self.sprite0.target().width;
                        self.sprite0.target_mut().y = y as f32 * self.sprite0.target().height;
                        self.batch.draw_sprite(&self.sprite0);
                    } else {
                        self.sprite.target_mut().x = x as f32 * self.sprite.target().width;
                        self.sprite.target_mut().y = y as f32 * self.sprite.target().height;
                        self.batch.draw_sprite(&self.sprite);
                    }
                }
            }
        }

        self.batch.end();
    }
}

fn main() {
    adore::logger::init(adore::logger::Filter::default());
    adore::Adore::new(adore::AdoreConfig::default()).run(App::new());
}

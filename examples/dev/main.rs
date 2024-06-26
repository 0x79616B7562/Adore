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

        let sprite = adore::Sprite::new(adore::load_texture_from_bytes(include_bytes!("dev.png")).unwrap());
        let sprite0 = adore::Sprite::new(adore::load_texture_from_bytes(include_bytes!("test.png")).unwrap());

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
    fn resize(&mut self, size: adore::Size<u32>) {
        self.batch.resize(size);
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
            self.throttle = 0.2;

            adore::set_title(format!(
                "FPS: {:.2}, TOTAL: {:.2}, DRAW CALLS: {}",
                self.fps.iter().sum::<f32>() / self.fps.len() as f32,
                game_time.total(),
                self.batch.draw_calls(),
            ));

            self.fps.clear();
        }
    }

    fn draw(&mut self, _game_time: adore::GameTime) {
        self.batch.begin().unwrap();

        #[allow(clippy::all)]
        for x in 0..10 {
            for y in 0..10 {
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

        self.batch.end().unwrap();
    }
}

fn main() {
    adore::logger::init(adore::logger::Filter::default());
    adore::Adore::new(adore::AdoreConfig::default()).run(App::new());
}

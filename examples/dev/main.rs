use std::path::Path;

struct App {
    batch: amber::Batch,
    sprite: amber::Sprite,

    fps: Vec<f32>,
    throttle: f32,
}

impl App {
    pub fn new() -> Self {
        let batch = amber::Batch::new();

        let sprite = amber::Sprite::new(amber::AssetManager::load_texture(Path::new("examples/dev/test.png")).unwrap());

        Self {
            batch,
            sprite,

            fps: vec![],
            throttle: 0.0,
        }
    }
}

impl amber::App for App {
    fn resize(&mut self, _size: amber::Size<u32>) {
    }

    fn update(&mut self, game_time: amber::GameTime) {
        if amber::input().key_just_pressed(amber::KeyCode::Escape) {
            amber::abort();
        }

        if self.throttle > 0.0 {
            self.throttle -= game_time.delta();
            self.fps.push(1.0 / game_time.delta());
        } else {
            self.fps.push(1.0 / game_time.delta());
            self.throttle = 1.0;
            amber::log::trace!(
                "FPS: {:.2}, TOTAL: {:.2}",
                self.fps.iter().sum::<f32>() / self.fps.len() as f32,
                game_time.total()
            );
            self.fps.clear();
        }
    }
    
    fn draw(&mut self) {
        self.batch.begin();

        for x in 0..10 {
            for y in 0..10 {
                self.sprite.target_mut().x = x as f32 * 32.0;
                self.sprite.target_mut().y = y as f32 * 32.0;

                self.batch.draw_sprite(&self.sprite);
            }
        }

        self.batch.end();
    }
}

fn main() {
    amber::logger::init(amber::logger::Filter::default());
    amber::Amber::new(amber::AmberConfig::default()).run(App::new());
}

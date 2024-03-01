// use std::path::Path;
//
// struct App {
//     batch: adore::Batch,
//     sprite: adore::Sprite,
//
//     fps: Vec<f32>,
//     throttle: f32,
// }
//
// impl App {
//     pub fn new() -> Self {
//         let batch = adore::Batch::new();
//
//         let sprite = adore::Sprite::new(adore::AssetManager::load_texture(Path::new("examples/dev/test.png")).unwrap());
//
//         Self {
//             batch,
//             sprite,
//
//             fps: vec![],
//             throttle: 0.0,
//         }
//     }
// }
//
// impl adore::Game for App {
//     fn resize(&mut self, _size: adore::Size<u32>) {
//     }
//
//     fn update(&mut self, game_time: adore::GameTime) {
//         if adore::input().key_just_pressed(adore::KeyCode::Escape) {
//             adore::abort();
//         }
//
//         if self.throttle > 0.0 {
//             self.throttle -= game_time.delta();
//             self.fps.push(1.0 / game_time.delta());
//         } else {
//             self.fps.push(1.0 / game_time.delta());
//             self.throttle = 1.0;
//             adore::log::trace!(
//                 "FPS: {:.2}, TOTAL: {:.2}",
//                 self.fps.iter().sum::<f32>() / self.fps.len() as f32,
//                 game_time.total()
//             );
//             self.fps.clear();
//         }
//     }
//
//     fn draw(&mut self) {
//         self.batch.begin();
//
//         for x in 0..10 {
//             for y in 0..10 {
//                 self.sprite.target_mut().x = x as f32 * 32.0;
//                 self.sprite.target_mut().y = y as f32 * 32.0;
//
//                 self.batch.draw_sprite(&self.sprite);
//             }
//         }
//
//         self.batch.end();
//     }
// }

fn main() {
    // adore::logger::init(adore::logger::Filter::default());
    // adore::Adore::new(adore::AdoreConfig::default()).run(App::new());
}

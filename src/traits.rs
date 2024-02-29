use crate::{
    time::GameTime,
    types::Size,
};

pub trait App {
    fn resize(&mut self, size: Size<u32>);
    fn update(&mut self, game_time: GameTime);
    fn draw(&mut self);
}

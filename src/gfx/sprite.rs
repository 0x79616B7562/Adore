use crate::{
    Color,
    Rectangle,
    Texture,
};

#[derive(Debug)]
pub struct Sprite {
    texture: Texture,

    target: Rectangle<f32>,
    color: Color<f32>,
}

impl Sprite {
    pub fn new(texture: Texture) -> Self {
        let target = Rectangle::new(0.0, 0.0, texture.width() as f32, texture.height() as f32);

        Self {
            texture,

            target,
            color: Color::default(),
        }
    }

    pub fn width(&self) -> u32 {
        self.texture.width()
    }

    pub fn height(&self) -> u32 {
        self.texture.height()
    }

    pub fn texture(&self) -> &Texture {
        &self.texture
    }

    pub fn target(&self) -> Rectangle<f32> {
        self.target
    }

    pub fn target_mut(&mut self) -> &mut Rectangle<f32> {
        &mut self.target
    }

    pub fn set_target(&mut self, target: Rectangle<f32>) {
        self.target = target;
    }

    pub fn color(&self) -> Color<f32> {
        self.color
    }

    pub fn color_mut(&mut self) -> &mut Color<f32> {
        &mut self.color
    }

    pub fn set_color(&mut self, color: Color<f32>) {
        self.color = color;
    }
}

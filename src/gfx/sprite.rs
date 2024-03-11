use std::rc::Rc;

use crate::{
    raw,
    Color,
    Rectangle,
};

#[derive(Debug)]
pub struct Sprite {
    texture: Rc<raw::Texture>,

    target: Rectangle<f32>,
    color: Color<f32>,
}

impl Sprite {
    pub fn new(texture: raw::Texture) -> Self {
        let target = Rectangle::new(0.0, 0.0, texture.dimensions.0 as f32, texture.dimensions.1 as f32);

        Self {
            texture: Rc::new(texture),

            target,
            color: Color::default(),
        }
    }

    pub fn width(&self) -> u32 {
        self.texture.dimensions.0
    }

    pub fn height(&self) -> u32 {
        self.texture.dimensions.1
    }

    pub(crate) fn clone_texture_rc(&self) -> Rc<raw::Texture> {
        Rc::clone(&self.texture)
    }

    pub fn texture(&self) -> &raw::Texture {
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

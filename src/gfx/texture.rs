use crate::raw;

#[derive(Debug)]
pub struct Texture {
    raw: raw::Texture,
}

impl Texture {
    pub fn new(raw: raw::Texture) -> Self {
        Self {
            raw,
        }
    }

    pub fn width(&self) -> u32 {
        self.raw.dimensions.0
    }

    pub fn height(&self) -> u32 {
        self.raw.dimensions.1
    }

    pub fn raw(&self) -> &raw::Texture {
        &self.raw
    }
}

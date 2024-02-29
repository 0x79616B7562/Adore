use std::{
    fs,
    path::Path,
};

use crate::{
    raw,
    Texture,
};

#[derive(Debug, Default)]
pub struct AssetManager;

impl AssetManager {
    pub fn load_texture(path: &Path) -> anyhow::Result<Texture> {
        let bytes = fs::read(path)?;
        let image = image::load_from_memory(&bytes)?;
        let rgba = image.to_rgba8();

        use image::GenericImageView;
        let dimensions = image.dimensions();

        Ok(Texture::new(raw::Texture::new(&rgba, dimensions, raw::TextureConfig {
            ..Default::default()
        })))
    }
}

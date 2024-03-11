use std::{
    fs,
    path::Path,
};

use crate::raw;

pub async fn load_texture_async(path: &Path) -> anyhow::Result<raw::Texture> {
    load_texture(path)
}

pub fn load_texture(path: &Path) -> anyhow::Result<raw::Texture> {
    let bytes = fs::read(path)?;
    let image = image::load_from_memory(&bytes)?;
    let rgba = image.to_rgba8();

    use image::GenericImageView;
    let dimensions = image.dimensions();

    Ok(raw::Texture::new(&rgba, dimensions, raw::TextureConfig {
        ..Default::default()
    }))
}

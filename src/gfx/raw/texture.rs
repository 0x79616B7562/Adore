use crate::gfx::raw::{
    AddressMode,
    CompareFunction,
    FilterMode,
    SamplerBorderColor,
    TextureConfig,
};

#[derive(Debug)]
pub struct Texture {
    pub(crate) bind_group: wgpu::BindGroup,
    pub(crate) dimensions: (u32, u32),
}

impl Texture {
    pub fn new(bytes: &[u8], dimensions: (u32, u32), config: TextureConfig) -> Self {
        let texture_size = wgpu::Extent3d {
            width: dimensions.0,
            height: dimensions.1,
            depth_or_array_layers: 1,
        };

        let texture = ctx!().device.create_texture(&wgpu::TextureDescriptor {
            size: texture_size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            label: Some("Texture"),
            view_formats: &[],
        });

        ctx!().queue.write_texture(
            wgpu::ImageCopyTexture {
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            bytes,
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(4 * dimensions.0),
                rows_per_image: Some(dimensions.1),
            },
            texture_size,
        );

        let texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        let sampler = ctx!().device.create_sampler(&wgpu::SamplerDescriptor {
            label: None,
            address_mode_u: match config.address_mode_u {
                AddressMode::ClampToEdge => wgpu::AddressMode::ClampToEdge,
                AddressMode::Repeat => wgpu::AddressMode::Repeat,
                AddressMode::MirrorRepeat => wgpu::AddressMode::MirrorRepeat,
                AddressMode::ClampToBorder => wgpu::AddressMode::ClampToBorder,
            },
            address_mode_v: match config.address_mode_v {
                AddressMode::ClampToEdge => wgpu::AddressMode::ClampToEdge,
                AddressMode::Repeat => wgpu::AddressMode::Repeat,
                AddressMode::MirrorRepeat => wgpu::AddressMode::MirrorRepeat,
                AddressMode::ClampToBorder => wgpu::AddressMode::ClampToBorder,
            },
            address_mode_w: match config.address_mode_w {
                AddressMode::ClampToEdge => wgpu::AddressMode::ClampToEdge,
                AddressMode::Repeat => wgpu::AddressMode::Repeat,
                AddressMode::MirrorRepeat => wgpu::AddressMode::MirrorRepeat,
                AddressMode::ClampToBorder => wgpu::AddressMode::ClampToBorder,
            },
            mag_filter: match config.mag_filter {
                FilterMode::Nearest => wgpu::FilterMode::Nearest,
                FilterMode::Linear => wgpu::FilterMode::Linear,
            },
            min_filter: match config.min_filter {
                FilterMode::Nearest => wgpu::FilterMode::Nearest,
                FilterMode::Linear => wgpu::FilterMode::Linear,
            },
            mipmap_filter: match config.mipmap_filter {
                FilterMode::Nearest => wgpu::FilterMode::Nearest,
                FilterMode::Linear => wgpu::FilterMode::Linear,
            },
            lod_min_clamp: config.lod_min_clamp,
            lod_max_clamp: config.lod_max_clamp,
            compare: if config.compare.is_some() {
                Some(match config.compare.unwrap() {
                    CompareFunction::Never => wgpu::CompareFunction::Never,
                    CompareFunction::Less => wgpu::CompareFunction::Less,
                    CompareFunction::Equal => wgpu::CompareFunction::Equal,
                    CompareFunction::LessEqual => wgpu::CompareFunction::LessEqual,
                    CompareFunction::Greater => wgpu::CompareFunction::Greater,
                    CompareFunction::NotEqual => wgpu::CompareFunction::NotEqual,
                    CompareFunction::GreaterEqual => wgpu::CompareFunction::GreaterEqual,
                    CompareFunction::Always => wgpu::CompareFunction::Always,
                })
            } else {
                None
            },
            anisotropy_clamp: config.anisotropy_clamp,
            border_color: if config.border_color.is_some() {
                Some(match config.border_color.unwrap() {
                    SamplerBorderColor::TransparentBlack => wgpu::SamplerBorderColor::TransparentBlack,
                    SamplerBorderColor::OpaqueBlack => wgpu::SamplerBorderColor::OpaqueBlack,
                    SamplerBorderColor::OpaqueWhite => wgpu::SamplerBorderColor::OpaqueWhite,
                    SamplerBorderColor::Zero => wgpu::SamplerBorderColor::Zero,
                })
            } else {
                None
            },
        });

        let bind_group = ctx!().device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &Texture::bind_group_layout(),
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&texture_view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&sampler),
                },
            ],
            label: Some("Texture Bind Group"),
        });

        Self {
            bind_group,
            dimensions,
        }
    }

    pub fn bind_group_layout() -> wgpu::BindGroupLayout {
        ctx!().device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        multisampled: false,
                        view_dimension: wgpu::TextureViewDimension::D2,
                        sample_type: wgpu::TextureSampleType::Float {
                            filterable: true,
                        },
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
            label: Some("Texture Bind Group Layout"),
        })
    }
}

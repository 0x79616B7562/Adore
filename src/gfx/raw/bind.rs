use std::num::{
    NonZeroU32,
    NonZeroU64,
};

use crate::gfx::raw::{
    BindGroupEntry,
    BindGroupLayoutEntry,
    Resource,
    ShaderStages,
};

pub struct Bind {
    pub(crate) bind_group: wgpu::BindGroup,
}

pub fn create_bind_group(layout: &wgpu::BindGroupLayout, entries: &[BindGroupEntry]) -> Bind {
    let mut __entries = vec![];

    for entry in entries {
        __entries.push(wgpu::BindGroupEntry {
            binding: entry.binding,
            resource: if entry.size.is_none() {
                match entry.resource {
                    Resource::Uniform(uniform) => uniform.as_entire_binding(),
                }
            } else {
                match entry.resource {
                    Resource::Uniform(uniform) => wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                        buffer: uniform,
                        offset: 0,
                        size: Some(NonZeroU64::new(entry.size.unwrap()).expect("Bind group size cant be 0")),
                    }),
                }
            },
        });
    }

    Bind {
        bind_group: ctx!().device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout,
            entries: &__entries,
            label: Some("Bind Group"),
        }),
    }
}

pub fn create_bind_group_layout(entries: &[BindGroupLayoutEntry]) -> wgpu::BindGroupLayout {
    let mut __entries = vec![];

    for entry in entries {
        __entries.push(wgpu::BindGroupLayoutEntry {
            binding: entry.binding,
            visibility: match entry.visibility {
                ShaderStages::None => wgpu::ShaderStages::NONE,
                ShaderStages::Vertex => wgpu::ShaderStages::VERTEX,
                ShaderStages::Fragment => wgpu::ShaderStages::FRAGMENT,
                ShaderStages::Compute => wgpu::ShaderStages::COMPUTE,
                ShaderStages::VertexFragment => wgpu::ShaderStages::VERTEX_FRAGMENT,
            },
            ty: match &entry.ty {
                crate::gfx::raw::BindingType::Uniform(has_dynamic_offset) => wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: *has_dynamic_offset,
                    min_binding_size: None,
                },
                crate::gfx::raw::BindingType::Texture {
                    sample_type,
                    view_dimension,
                    multisampled,
                } => wgpu::BindingType::Texture {
                    sample_type: match sample_type {
                        crate::gfx::raw::TextureSampleType::Float {
                            filterable,
                        } => wgpu::TextureSampleType::Float {
                            filterable: *filterable,
                        },
                        crate::gfx::raw::TextureSampleType::Depth => wgpu::TextureSampleType::Depth,
                        crate::gfx::raw::TextureSampleType::Sint => wgpu::TextureSampleType::Sint,
                        crate::gfx::raw::TextureSampleType::Uint => wgpu::TextureSampleType::Uint,
                    },
                    view_dimension: match view_dimension {
                        crate::gfx::raw::TextureViewDimension::D1 => wgpu::TextureViewDimension::D1,
                        crate::gfx::raw::TextureViewDimension::D2 => wgpu::TextureViewDimension::D2,
                        crate::gfx::raw::TextureViewDimension::D2Array => wgpu::TextureViewDimension::D2Array,
                        crate::gfx::raw::TextureViewDimension::Cube => wgpu::TextureViewDimension::Cube,
                        crate::gfx::raw::TextureViewDimension::CubeArray => wgpu::TextureViewDimension::CubeArray,
                        crate::gfx::raw::TextureViewDimension::D3 => wgpu::TextureViewDimension::D3,
                    },
                    multisampled: *multisampled,
                },
                crate::gfx::raw::BindingType::Sampler(sampler) => wgpu::BindingType::Sampler(match sampler {
                    crate::gfx::raw::SamplerBindingType::Filtering => wgpu::SamplerBindingType::Filtering,
                    crate::gfx::raw::SamplerBindingType::NonFiltering => wgpu::SamplerBindingType::NonFiltering,
                    crate::gfx::raw::SamplerBindingType::Comparison => wgpu::SamplerBindingType::Comparison,
                }),
            },
            count: if entry.count.is_none() {
                None
            } else {
                Some(NonZeroU32::new(entry.count.unwrap()).unwrap())
            },
        });
    }

    ctx!().device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        entries: &__entries,
        label: Some("Bind Group Layout"),
    })
}

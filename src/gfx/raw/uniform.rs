use crate::gfx::raw::{
    bind::{
        self,
        Bind,
    },
    BindGroupEntry,
    BindGroupLayoutEntry,
    BindingType,
    Resource,
    ShaderStages,
};

#[derive(Debug)]
pub struct Uniform {
    pub(crate) buffer: wgpu::Buffer,
    pub(crate) bind_group: Bind,
}

impl Uniform {
    pub fn new(data: &[u8], stages: ShaderStages) -> Self {
        let buffer = ctx!().device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Uniform Buffer"),
            size: data.len() as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        ctx!().queue.write_buffer(&buffer, 0, data);

        let bind_group_layout = Self::bind_group_layout(stages);

        let bind_group = bind::create_bind_group(&bind_group_layout, &[BindGroupEntry {
            binding: 0,
            resource: Resource::Uniform(&buffer),
            size: None,
        }]);

        Self {
            buffer,
            bind_group,
        }
    }

    #[inline]
    pub fn set(&self, data: &[u8]) {
        ctx!().queue.write_buffer(&self.buffer, 0, data);
    }

    pub fn bind_group_layout(stages: ShaderStages) -> wgpu::BindGroupLayout {
        bind::create_bind_group_layout(&[BindGroupLayoutEntry {
            binding: 0,
            visibility: stages,
            ty: BindingType::Uniform(false),
            count: None,
        }])
    }
}

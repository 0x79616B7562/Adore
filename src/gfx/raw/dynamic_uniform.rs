use hashbrown::HashMap;
use once_cell::sync::Lazy;
use uuid::Uuid;

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

//

#[allow(clippy::all)]
static mut DYNAMIC_UNIFORMS_RESET_QUEUE: Lazy<HashMap<u128, *mut DynamicUniform>> = Lazy::new(|| HashMap::new());

pub(crate) fn reset_dynamic_uniforms() {
    unsafe {
        if DYNAMIC_UNIFORMS_RESET_QUEUE.is_empty() {
            return;
        }

        for durq in DYNAMIC_UNIFORMS_RESET_QUEUE.values_mut() {
            (**durq).reset();
        }

        DYNAMIC_UNIFORMS_RESET_QUEUE.clear();
    }
}

//

pub struct DynamicUniform {
    uuid: u128,

    pub(crate) buffer: wgpu::Buffer,
    pub(crate) bind_groups: Vec<Bind>,

    size: usize,
    length: u32,
    offset: u32,
    desired_length: u32,

    visibility: ShaderStages,
}

impl Drop for DynamicUniform {
    fn drop(&mut self) {
        unsafe {
            if DYNAMIC_UNIFORMS_RESET_QUEUE.contains_key(&self.uuid) {
                DYNAMIC_UNIFORMS_RESET_QUEUE.remove(&self.uuid);
            }
        }
    }
}

impl DynamicUniform {
    pub fn new(data: &[u8], stages: ShaderStages) -> Self {
        Self::with_capacity(data, 1, stages)
    }

    pub fn with_capacity(data: &[u8], length: u32, stages: ShaderStages) -> Self {
        let size = std::mem::size_of_val(data);

        let buffer = ctx!().device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Dynamic Uniform Buffer"),
            size: Self::step_u64(size) * length as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::COPY_SRC,
            mapped_at_creation: false,
        });

        ctx!().queue.write_buffer(&buffer, 0, data);

        let bind_group = bind::create_bind_group(&Self::bind_group_layout(stages), &[BindGroupEntry {
            binding: 0,
            resource: Resource::Uniform(&buffer),
            size: Some(size as u64),
        }]);

        Self {
            uuid: Uuid::new_v4().as_u128(),

            buffer,
            bind_groups: vec![bind_group],

            size,
            length,
            offset: 0,
            desired_length: 0,

            visibility: stages,
        }
    }

    pub fn resize(&mut self, length: u32) {
        match self.length.cmp(&length) {
            std::cmp::Ordering::Less => {
                self.length = length;
                self.expand();
            },
            std::cmp::Ordering::Equal => (),
            std::cmp::Ordering::Greater => {
                self.length = length;
                self.shrink();
            },
        }
    }

    #[inline]
    fn shrink(&mut self) {
        let buffer = ctx!().device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Dynamic Uniform Buffer"),
            size: Self::step_u64(self.size) * self.length as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::COPY_SRC,
            mapped_at_creation: false,
        });

        let mut encoder = ctx!().create_encoder();

        encoder.copy_buffer_to_buffer(&self.buffer, 0, &buffer, 0, buffer.size());

        ctx!().queue.submit(Some(encoder.finish()));

        self.buffer = buffer;

        self.bind_groups
            .push(bind::create_bind_group(&Self::bind_group_layout(self.visibility), &[
                BindGroupEntry {
                    binding: 0,
                    resource: Resource::Uniform(&self.buffer),
                    size: Some(self.size as u64),
                },
            ]));
    }

    #[inline]
    fn expand(&mut self) {
        let buffer = ctx!().device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Dynamic Uniform Buffer"),
            size: Self::step_u64(self.size) * self.length as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::COPY_SRC,
            mapped_at_creation: false,
        });

        let mut encoder = ctx!().create_encoder();

        encoder.copy_buffer_to_buffer(&self.buffer, 0, &buffer, 0, self.buffer.size());

        ctx!().queue.submit(Some(encoder.finish()));

        self.buffer = buffer;

        self.bind_groups
            .push(bind::create_bind_group(&Self::bind_group_layout(self.visibility), &[
                BindGroupEntry {
                    binding: 0,
                    resource: Resource::Uniform(&self.buffer),
                    size: Some(self.size as u64),
                },
            ]));
    }

    #[inline]
    pub fn set(&mut self, data: &[u8]) {
        if self.offset + 1 > self.length {
            self.length += 1;
            self.expand();
        }

        ctx!()
            .queue
            .write_buffer(&self.buffer, Self::step_u64(self.size) * self.offset as u64, data);

        self.offset += 1;
        self.desired_length += 1;

        unsafe {
            if !DYNAMIC_UNIFORMS_RESET_QUEUE.contains_key(&self.uuid) {
                DYNAMIC_UNIFORMS_RESET_QUEUE.insert(self.uuid, self);
            }
        }
    }

    pub(crate) fn offset_of(&self, index: u32) -> u32 {
        Self::step_u64(self.size) as u32 * index
    }

    pub fn bind_group_layout(stages: ShaderStages) -> wgpu::BindGroupLayout {
        bind::create_bind_group_layout(&[BindGroupLayoutEntry {
            binding: 0,
            visibility: stages,
            ty: BindingType::Uniform(true),
            count: None,
        }])
    }

    fn step_u64(value: usize) -> u64 {
        Self::step_u32(value) as u64
    }

    fn step_u32(value: usize) -> u32 {
        let step = ctx!().device.limits().min_uniform_buffer_offset_alignment;

        let divide_and_ceil = value as u32 / step
            + if value as u32 % step == 0 {
                0
            } else {
                1
            };

        step * divide_and_ceil
    }

    pub(crate) fn offset(&self) -> u32 {
        self.offset
    }

    pub(crate) fn reset(&mut self) {
        self.offset = 0;

        if self.desired_length != self.length {
            self.resize(self.desired_length);
        }

        self.desired_length = 0;

        if self.bind_groups.len() > 1 {
            self.bind_groups.remove(0);
        }
    }

    pub fn is_empty(&self) -> bool {
        self.length == 0
    }

    pub fn len(&self) -> u32 {
        self.length
    }
}

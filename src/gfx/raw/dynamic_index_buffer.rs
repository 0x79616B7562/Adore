use wgpu::util::DeviceExt;

use crate::gfx::raw::IndexFormat;

pub struct DynamicIndexBuffer {
    pub(crate) buffer: wgpu::Buffer,
    pub(crate) format: wgpu::IndexFormat,

    size: usize,

    len: u32,
}

impl DynamicIndexBuffer {
    pub fn new(data: &[u8], format: IndexFormat, len: usize) -> Self {
        let buffer = ctx!().device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Dynamic Index Buffer"),
            contents: data,
            usage: wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST,
        });

        Self {
            buffer,
            format: match format {
                IndexFormat::Uint16 => wgpu::IndexFormat::Uint16,
                IndexFormat::Uint32 => wgpu::IndexFormat::Uint32,
            },

            size: data.len(),

            len: len as u32,
        }
    }

    pub fn set(&mut self, data: &[u8], len: usize) {
        if data.len() != self.size {
            self.buffer = ctx!().device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Dynamic Index Buffer"),
                contents: data,
                usage: wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST,
            });

            self.len = len as u32;
            self.size = data.len();
        }

        ctx!().queue.write_buffer(&self.buffer, 0, data);
    }

    #[allow(clippy::all)]
    pub fn len(&self) -> u32 {
        self.len
    }
}

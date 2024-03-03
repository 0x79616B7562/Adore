use wgpu::util::DeviceExt;

use crate::gfx::raw::IndexFormat;

#[derive(Debug)]
pub struct IndexBuffer {
    pub(crate) buffer: wgpu::Buffer,
    pub(crate) format: wgpu::IndexFormat,

    len: u32,
}

impl IndexBuffer {
    pub fn new(data: &[u8], format: IndexFormat, len: usize) -> Self {
        let buffer = ctx!().device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: data,
            usage: wgpu::BufferUsages::INDEX,
        });

        IndexBuffer {
            buffer,
            format: match format {
                IndexFormat::Uint16 => wgpu::IndexFormat::Uint16,
                IndexFormat::Uint32 => wgpu::IndexFormat::Uint32,
            },

            len: len as u32,
        }
    }

    #[allow(clippy::all)]
    pub fn len(&self) -> u32 {
        self.len
    }
}

use wgpu::util::DeviceExt;

pub struct VertexBuffer {
    pub(crate) buffer: wgpu::Buffer,
}

impl VertexBuffer {
    pub fn new(data: &[u8]) -> Self {
        let buffer = ctx!().device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: data,
            usage: wgpu::BufferUsages::VERTEX,
        });

        Self {
            buffer,
        }
    }
}

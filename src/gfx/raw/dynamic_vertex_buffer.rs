use wgpu::util::DeviceExt;

pub struct DynamicVertexBuffer {
    pub(crate) buffer: wgpu::Buffer,

    size: usize,
}

impl DynamicVertexBuffer {
    pub fn new(data: &[u8]) -> Self {
        let buffer = ctx!().device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Dynamic Vertex Buffer"),
            contents: data,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
        });

        Self {
            buffer,

            size: data.len(),
        }
    }

    pub fn set(&mut self, data: &[u8]) {
        if data.len() != self.size {
            self.buffer = ctx!().device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Dynamic Vertex Buffer"),
                contents: data,
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            });

            self.size = data.len();
        }

        ctx!().queue.write_buffer(&self.buffer, 0, data);
    }
}

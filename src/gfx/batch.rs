use crate::{
    raw,
    Color,
    Rectangle,
    Sprite,
};

#[repr(C)]
#[derive(Debug, Clone, Copy, crate::Desc)]
struct Vertex {
    #[f32x2(0)]
    position: [f32; 2],
    #[f32x4(1)]
    color: [f32; 4],
    #[f32x2(2)]
    texcoord: [f32; 2],
}

//

pub struct Batch {
    pipeline: raw::Pipeline,
    rp: Option<raw::RenderPass<'static>>,

    camera_uniform: raw::Uniform,

    vb: Vec<raw::VertexBuffer>,
    ib: Vec<raw::IndexBuffer>,

    vertex: Vec<Vertex>,
    index: Vec<u32>,
    index_offset: u32,

    current_texture: *const Sprite,
}

impl Default for Batch {
    fn default() -> Self {
        let pipeline = raw::Pipeline::new(&raw::PipelineConfig {
            shader_source: include_str_from_root!("shaders/batch.wgsl"),
            vertex_buffer_layouts: &[Vertex::desc()],
            bind_group_layouts: &[
                &raw::Texture::bind_group_layout(),
                &raw::Uniform::bind_group_layout(raw::ShaderStages::Vertex),
            ],
            depth_stencil_write_enabled: false,
        });

        let view = glam::Mat4::look_at_rh(glam::Vec3::new(0.0, 0.0, 1.0), glam::Vec3::new(0.0, 0.0, 0.0), glam::Vec3::Y);
        let proj = glam::Mat4::orthographic_rh(0.0, 1280.0, 0.0, 720.0, 0.0, 1.0);

        let camera_uniform = raw::Uniform::new(
            crate::cast(&(proj * view).to_cols_array()),
            raw::ShaderStages::Vertex,
        );

        Self {
            pipeline,
            rp: None,

            camera_uniform,

            vb: vec![],
            ib: vec![],

            vertex: vec![],
            index: vec![],
            index_offset: 0,

            current_texture: std::ptr::null(),
        }
    }
}

impl Batch {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn begin(&mut self) {
        if self.rp.is_some() {
            log::error!("Batch is already drawing");

            return;
        }

        if let Some(frame) = raw::frame() {
            self.vb.clear();
            self.ib.clear();

            self.rp = Some(frame.create_render_pass(false));
        } else {
            log::error!("Frame is None");
        }
    }

    pub fn end(&mut self) {
        if self.rp.is_none() {
            log::error!("Batch is not drawing");

            return;
        }

        if self.vertex.is_empty() || self.index.is_empty() {
            _ = self.rp.take();
            return;
        }

        self.draw_call();

        _ = self.rp.take();
        self.current_texture = std::ptr::null();
    }

    fn draw_call(&mut self) {
        self.rp.as_mut().unwrap().set_pipeline(&self.pipeline);
        self.rp.as_mut().unwrap().set_uniform(1, &self.camera_uniform);

        let vb = raw::VertexBuffer::new(crate::cast(&self.vertex));
        let ib = raw::IndexBuffer::new(crate::cast(&self.index), raw::IndexFormat::Uint32, self.index.len());

        self.rp.as_mut().unwrap().set_vertex_buffer(0, &vb);
        self.rp.as_mut().unwrap().set_index_buffer(&ib);
        self.rp.as_mut().unwrap().draw_indexed(0..ib.len(), 0, 0..1);

        self.vb.push(vb);
        self.ib.push(ib);

        self.vertex.clear();
        self.index.clear();
        self.index_offset = 0;
    }

    fn add_quad(&mut self, target: Rectangle<f32>, color: Color<f32>) {
        self.vertex.extend_from_slice(&[
            Vertex {
                position: [target.x, target.y],
                color: color.into(),
                texcoord: [1.0, 1.0],
            },
            Vertex {
                position: [target.x, target.y + target.height],
                color: color.into(),
                texcoord: [1.0, 0.0],
            },
            Vertex {
                position: [target.x + target.width, target.y + target.height],
                color: color.into(),
                texcoord: [0.0, 0.0],
            },
            Vertex {
                position: [target.x + target.width, target.y],
                color: color.into(),
                texcoord: [0.0, 1.0],
            },
        ]);

        #[allow(clippy::all)]
        self.index.extend_from_slice(&[
            0 + self.index_offset,
            2 + self.index_offset,
            1 + self.index_offset,
            0 + self.index_offset,
            3 + self.index_offset,
            2 + self.index_offset,
        ]);

        self.index_offset += 4;
    }

    pub fn draw_sprite(&mut self, sprite: &Sprite) {
        if self.current_texture != sprite as _ {
            if !self.current_texture.is_null() {
                self.draw_call();
            }

            self.current_texture = sprite as _;
        
            self.rp.as_mut().unwrap().set_texture(0, sprite.texture().raw());
        }

        self.add_quad(sprite.target(), sprite.color());
    }
}

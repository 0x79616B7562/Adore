use std::ops::Range;

use crate::gfx::raw::{
    DynamicIndexBuffer,
    DynamicUniform,
    DynamicVertexBuffer,
    IndexBuffer,
    Pipeline,
    Texture,
    Uniform,
    VertexBuffer,
};

#[derive(Debug)]
pub struct RenderPass<'a> {
    pub render_pass: wgpu::RenderPass<'a>,
}

impl<'a> RenderPass<'a> {
    pub fn new(render_pass: wgpu::RenderPass<'a>) -> Self {
        Self {
            render_pass,
        }
    }

    #[inline]
    pub fn set_pipeline(&mut self, pipeline: &Pipeline) {
        let mut inner = |pipeline: *const Pipeline| unsafe {
            self.render_pass.set_pipeline(&(*pipeline).pipeline);
        };

        inner(pipeline);
    }

    #[inline]
    pub fn set_texture(&mut self, index: u32, texture: &Texture) {
        let mut inner = |index: u32, texture: *const Texture| unsafe {
            self.render_pass.set_bind_group(index, &(*texture).bind_group, &[]);
        };

        inner(index, texture);
    }

    #[inline]
    pub fn set_uniform(&mut self, index: u32, uniform: &Uniform) {
        let mut inner = |index: u32, uniform: *const Uniform| unsafe {
            self.render_pass.set_bind_group(index, &(*uniform).bind_group.bind_group, &[]);
        };

        inner(index, uniform);
    }

    #[inline]
    pub fn set_dynamic_uniform(&mut self, index: u32, uniform: &DynamicUniform) {
        let mut inner = |index: u32, uniform: *const DynamicUniform| unsafe {
            self.render_pass
                .set_bind_group(index, &(*uniform).bind_groups.last().unwrap().bind_group, &[(*uniform).offset_of(
                    if (*uniform).offset() == 0 {
                        0
                    } else {
                        (*uniform).offset() - 1
                    },
                )]);
        };

        inner(index, uniform);
    }

    #[inline]
    pub fn set_vertex_buffer(&mut self, index: u32, vertex_buffer: &VertexBuffer) {
        let mut inner = |index: u32, vertex_buffer: *const VertexBuffer| unsafe {
            self.render_pass.set_vertex_buffer(index, (*vertex_buffer).buffer.slice(..));
        };

        inner(index, vertex_buffer);
    }

    #[inline]
    pub fn set_index_buffer(&mut self, index_buffer: &IndexBuffer) {
        let mut inner = |index_buffer: *const IndexBuffer| unsafe {
            self.render_pass
                .set_index_buffer((*index_buffer).buffer.slice(..), (*index_buffer).format);
        };

        inner(index_buffer);
    }

    #[inline]
    pub fn set_dynamic_vertex_buffer(&mut self, index: u32, vertex_buffer: &DynamicVertexBuffer) {
        let mut inner = |index: u32, vertex_buffer: *const DynamicVertexBuffer| unsafe {
            self.render_pass.set_vertex_buffer(index, (*vertex_buffer).buffer.slice(..));
        };

        inner(index, vertex_buffer);
    }

    #[inline]
    pub fn set_dynamic_index_buffer(&mut self, index_buffer: &DynamicIndexBuffer) {
        let mut inner = |index_buffer: *const DynamicIndexBuffer| unsafe {
            self.render_pass
                .set_index_buffer((*index_buffer).buffer.slice(..), (*index_buffer).format);
        };

        inner(index_buffer);
    }

    #[inline]
    pub fn draw(&mut self, vertices: Range<u32>, instances: Range<u32>) {
        self.render_pass.draw(vertices, instances);
    }

    #[inline]
    pub fn draw_indexed(&mut self, indices: Range<u32>, base_vertes: i32, instances: Range<u32>) {
        self.render_pass.draw_indexed(indices, base_vertes, instances);
    }

    #[inline]
    pub fn set_viewport(&mut self, x: f32, y: f32, width: f32, height: f32) {
        self.render_pass.set_viewport(x, y, width, height, 0.0, 1.0);
    }
}

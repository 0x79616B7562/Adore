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

//

#[inline(always)]
unsafe fn extend_lifetime<'a, T>(t: *const T) -> &'a T {
    unsafe { &*t }
}

//

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
    pub fn set_pipeline(&mut self, pipeline: &'a Pipeline) {
        self.render_pass.set_pipeline(&pipeline.pipeline);
    }

    #[inline]
    pub fn set_texture(&mut self, index: u32, texture: &'a Texture) {
        self.render_pass.set_bind_group(index, &texture.bind_group, &[]);
    }

    #[inline]
    pub fn set_uniform(&mut self, index: u32, uniform: &'a Uniform) {
        self.render_pass.set_bind_group(index, &uniform.bind_group.bind_group, &[]);
    }

    #[inline]
    pub fn set_dynamic_uniform(&mut self, index: u32, uniform: &'a DynamicUniform) {
        self.render_pass
            .set_bind_group(index, &uniform.bind_groups.last().unwrap().bind_group, &[uniform.offset_of(
                if uniform.offset() == 0 {
                    0
                } else {
                    uniform.offset() - 1
                },
            )]);
    }

    #[inline]
    pub fn set_vertex_buffer(&mut self, index: u32, vertex_buffer: &'a VertexBuffer) {
        self.render_pass.set_vertex_buffer(index, vertex_buffer.buffer.slice(..));
    }

    #[inline]
    pub fn set_index_buffer(&mut self, index_buffer: &'a IndexBuffer) {
        self.render_pass
            .set_index_buffer(index_buffer.buffer.slice(..), index_buffer.format);
    }

    #[inline]
    pub fn set_dynamic_vertex_buffer(&mut self, index: u32, vertex_buffer: &'a DynamicVertexBuffer) {
        self.render_pass.set_vertex_buffer(index, vertex_buffer.buffer.slice(..));
    }

    #[inline]
    pub fn set_dynamic_index_buffer(&mut self, index_buffer: &'a DynamicIndexBuffer) {
        self.render_pass
            .set_index_buffer(index_buffer.buffer.slice(..), index_buffer.format);
    }

    #[inline]
    pub unsafe fn set_pipeline_unsafe(&mut self, pipeline: &Pipeline) {
        self.render_pass.set_pipeline(extend_lifetime(&pipeline.pipeline));
    }

    #[inline]
    pub unsafe fn set_texture_unsafe(&mut self, index: u32, texture: &Texture) {
        self.render_pass.set_bind_group(index, extend_lifetime(&texture.bind_group), &[]);
    }

    #[inline]
    pub unsafe fn set_uniform_unsafe(&mut self, index: u32, uniform: &Uniform) {
        self.render_pass
            .set_bind_group(index, extend_lifetime(&uniform.bind_group.bind_group), &[]);
    }

    #[inline]
    pub unsafe fn set_dynamic_uniform_unsafe(&mut self, index: u32, uniform: &DynamicUniform) {
        self.render_pass
            .set_bind_group(index, extend_lifetime(&uniform.bind_groups.last().unwrap().bind_group), &[uniform
                .offset_of(if uniform.offset() == 0 {
                    0
                } else {
                    uniform.offset() - 1
                })]);
    }

    #[inline]
    pub unsafe fn set_vertex_buffer_unsafe(&mut self, index: u32, vertex_buffer: &VertexBuffer) {
        self.render_pass
            .set_vertex_buffer(index, extend_lifetime(vertex_buffer).buffer.slice(..));
    }

    #[inline]
    pub unsafe fn set_index_buffer_unsafe(&mut self, index_buffer: &IndexBuffer) {
        self.render_pass
            .set_index_buffer(extend_lifetime(index_buffer).buffer.slice(..), index_buffer.format);
    }

    #[inline]
    pub unsafe fn set_dynamic_vertex_buffer_unsafe(&mut self, index: u32, vertex_buffer: &DynamicVertexBuffer) {
        self.render_pass
            .set_vertex_buffer(index, extend_lifetime(vertex_buffer).buffer.slice(..));
    }

    #[inline]
    pub unsafe fn set_dynamic_index_buffer_unsafe(&mut self, index_buffer: &DynamicIndexBuffer) {
        self.render_pass
            .set_index_buffer(extend_lifetime(index_buffer).buffer.slice(..), index_buffer.format);
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

    pub fn raw(&'a mut self) -> &'a mut wgpu::RenderPass {
        &mut self.render_pass
    }
}

use crate::gfx::raw::{
    LoadOp,
    RenderPass,
};

pub struct Frame {
    pub output: wgpu::SurfaceTexture,
    pub view: wgpu::TextureView,
    pub encoder: wgpu::CommandEncoder,
}

impl Frame {
    pub(crate) fn new(output: wgpu::SurfaceTexture, view: wgpu::TextureView, encoder: wgpu::CommandEncoder) -> Self {
        Self {
            output,
            view,
            encoder,
        }
    }

    pub fn create_render_pass(&mut self, depth_write_enabled: bool) -> RenderPass {
        self.create_render_pass_with_load_op(depth_write_enabled, LoadOp::Load)
    }

    pub fn create_render_pass_with_load_op(&mut self, depth_write_enabled: bool, load_op: LoadOp) -> RenderPass {
        RenderPass::new(self.encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &self.view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: match load_op {
                        LoadOp::Clear(color) => wgpu::LoadOp::Clear(wgpu::Color {
                            r: color.r,
                            g: color.g,
                            b: color.b,
                            a: color.a,
                        }),
                        LoadOp::Load => wgpu::LoadOp::Load,
                    },
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: if depth_write_enabled {
                Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &ctx!().depth_texture.view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: wgpu::StoreOp::Store,
                    }),
                    stencil_ops: None,
                })
            } else {
                None
            },
            occlusion_query_set: None,
            timestamp_writes: None,
        }))
    }
}

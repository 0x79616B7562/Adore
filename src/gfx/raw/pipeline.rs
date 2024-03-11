use crate::gfx::raw::{
    Depth,
    PipelineConfig,
};

#[derive(Debug)]
pub struct Pipeline {
    pub(crate) pipeline: wgpu::RenderPipeline,
}

impl Pipeline {
    pub fn new(config: &PipelineConfig) -> Self {
        let shader = ctx!().device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(config.shader_source.into()),
        });

        let mut layouts = config.bind_group_layouts.to_vec();
        layouts.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
        let layouts = layouts.iter().map(|ol| ol.1).collect::<Vec<_>>();

        let render_pipeline_layout = ctx!().device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: &layouts,
            push_constant_ranges: &[],
        });

        let pipeline = ctx!().device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: config.vertex_buffer_layouts,
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: ctx!().config.format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                // cull_mode: None,
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: if config.depth_stencil_write_enabled {
                Some(wgpu::DepthStencilState {
                    format: Depth::DEPTH_FORMAT,
                    depth_write_enabled: true,
                    depth_compare: wgpu::CompareFunction::Less,
                    stencil: wgpu::StencilState::default(),
                    bias: wgpu::DepthBiasState::default(),
                })
            } else {
                None
            },
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        });

        Self {
            pipeline,
        }
    }
}

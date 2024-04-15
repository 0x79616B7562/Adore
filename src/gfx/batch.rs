use std::rc::Rc;

use crate::{
    errors::BatchError,
    raw,
    Color,
    Rectangle,
    Size,
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
    #[u32(3)]
    texture_index: u32,
}

//

#[derive(Debug)]
struct DrawCall {
    pub vb: Option<raw::VertexBuffer>,
    pub ib: Option<raw::IndexBuffer>,

    pub textures: Vec<Rc<raw::Texture>>,

    pub vertex: Vec<Vertex>,
    pub index: Vec<u32>,
    pub index_offset: u32,

    pub current_texture: *const raw::Texture,
}

impl Default for DrawCall {
    fn default() -> Self {
        Self {
            vb: None,
            ib: None,

            textures: vec![],

            vertex: vec![],
            index: vec![],
            index_offset: 0,

            current_texture: std::ptr::null(),
        }
    }
}

//

#[derive(Debug)]
pub struct Batch {
    pipeline: raw::Pipeline,
    camera_uniform: raw::Uniform,

    draw_calls: Vec<DrawCall>,

    is_drawing: bool,
    capacity: u32,
    blank_texture: raw::Texture,
}

impl Default for Batch {
    fn default() -> Self {
        let capacity = raw::device().limits().max_bind_groups - 1;

        log::debug!("Batch Texture Capacity: {:?}", capacity);

        //

        let mut bind_group_layouts = vec![(0, raw::Uniform::bind_group_layout(raw::ShaderStages::Vertex))];

        for i in 1..capacity + 1 {
            bind_group_layouts.push((i, raw::Texture::bind_group_layout()));
        }

        //

        let shader_source = {
            let mut bg = String::new();
            for i in 0..capacity {
                let index = i + 1;
                bg += format!(
                    r#"
@group({index}) @binding(0) var texture_{i}: texture_2d<f32>;
@group({index}) @binding(1) var texture_sampler_{i}: sampler;
                    "#
                )
                .as_str();
            }
            bg += "\n";

            let mut rets = String::new();
            for i in 0..capacity {
                rets += format!(
                    r#"
if (in.texture_index == {i}) {{
out = textureSample(texture_{i}, texture_sampler_{i}, in.texcoord) * in.color;
}}
                "#
                )
                .as_str();
            }
            rets += "\n";

            let shader_source = include_str_from_root!("res/shaders/batch_compatibility.wgsl").to_string();
            let shader_source = shader_source.replace("#include_body", rets.as_str());
            let shader_source = shader_source.replace("#include_bind_groups", bg.as_str());
            shader_source
        };

        //

        let pipeline = raw::Pipeline::new(raw::PipelineConfig {
            shader_source: shader_source.as_str(),
            vertex_buffer_layouts: &[Vertex::desc()],
            bind_group_layouts,
            depth_stencil_write_enabled: false,
        });

        //

        let view = glam::Mat4::look_at_rh(glam::Vec3::new(0.0, 0.0, 1.0), glam::Vec3::new(0.0, 0.0, 0.0), glam::Vec3::Y);
        let proj = glam::Mat4::orthographic_rh(0.0, 1280.0, 0.0, 720.0, 0.0, 1.0);

        let camera_uniform = raw::Uniform::new(crate::cast(&(proj * view).to_cols_array()), raw::ShaderStages::Vertex);

        //

        let blank_texture = raw::Texture::new(
            include_bytes_from_root!("res/textures/1x1white.jpg"),
            (1, 1),
            raw::TextureConfig::default(),
        );

        Self {
            pipeline,
            camera_uniform,

            draw_calls: vec![],

            is_drawing: false,
            capacity,
            blank_texture,
        }
    }
}

impl Batch {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn resize(&mut self, size: Size<u32>) {
        let view = glam::Mat4::look_at_rh(glam::Vec3::new(0.0, 0.0, 1.0), glam::Vec3::new(0.0, 0.0, 0.0), glam::Vec3::Y);
        let proj = glam::Mat4::orthographic_rh(0.0, size.width as f32, 0.0, size.height as f32, 0.0, 1.0);

        self.camera_uniform.set(crate::cast(&(proj * view).to_cols_array()));
    }

    pub fn begin(&mut self) -> anyhow::Result<()> {
        if self.is_drawing {
            return Err(anyhow::anyhow!(BatchError::BatchIsDrawing));
        }

        self.is_drawing = true;

        self.draw_calls.clear();

        Ok(())
    }

    pub fn end(&mut self) -> anyhow::Result<()> {
        if !self.is_drawing {
            return Err(anyhow::anyhow!(BatchError::BatchNotDrawing));
        }

        self.is_drawing = false;

        self.flush()
    }

    fn flush(&mut self) -> anyhow::Result<()> {
        if self.draw_calls.is_empty() {
            return Ok(());
        }

        match raw::frame() {
            Some(frame) => {
                let mut rp = frame.create_render_pass(false);

                rp.set_pipeline(&self.pipeline);
                rp.set_uniform(0, &self.camera_uniform);

                for draw_call in self.draw_calls.iter_mut() {
                    draw_call.vb = Some(raw::VertexBuffer::new(crate::cast(&draw_call.vertex)));
                    draw_call.ib = Some(raw::IndexBuffer::new(
                        crate::cast(&draw_call.index),
                        raw::IndexFormat::Uint32,
                        draw_call.index.len(),
                    ));

                    for i in 0..self.capacity {
                        rp.set_texture(i + 1, match draw_call.textures.get(i as usize) {
                            Some(texture) => texture,
                            None => &self.blank_texture,
                        });
                    }

                    rp.set_vertex_buffer(0, draw_call.vb.as_ref().unwrap());
                    rp.set_index_buffer(draw_call.ib.as_ref().unwrap());
                    rp.draw_indexed(0..draw_call.ib.as_ref().unwrap().len(), 0, 0..1);
                }

                Ok(())
            },
            None => Err(anyhow::anyhow!(BatchError::FrameIsNone)),
        }
    }

    fn add_quad(draw_call: &mut DrawCall, target: Rectangle<f32>, color: Color<f32>, texture_index: u32) {
        draw_call.vertex.extend_from_slice(&[
            Vertex {
                position: [target.x, target.y],
                color: color.into(),
                texcoord: [0.0, 1.0],
                texture_index,
            },
            Vertex {
                position: [target.x, target.y + target.height],
                color: color.into(),
                texcoord: [0.0, 0.0],
                texture_index,
            },
            Vertex {
                position: [target.x + target.width, target.y + target.height],
                color: color.into(),
                texcoord: [1.0, 0.0],
                texture_index,
            },
            Vertex {
                position: [target.x + target.width, target.y],
                color: color.into(),
                texcoord: [1.0, 1.0],
                texture_index,
            },
        ]);

        #[allow(clippy::all)]
        draw_call.index.extend_from_slice(&[
            0 + draw_call.index_offset,
            2 + draw_call.index_offset,
            1 + draw_call.index_offset,
            0 + draw_call.index_offset,
            3 + draw_call.index_offset,
            2 + draw_call.index_offset,
        ]);

        draw_call.index_offset += 4;
    }

    fn add_new_draw_call(&mut self, sprite: &Sprite, texture_index: u32) {
        let mut draw_call = DrawCall {
            textures: vec![sprite.clone_texture_rc()],
            ..Default::default()
        };

        draw_call.current_texture = sprite.texture() as _;
        Self::add_quad(&mut draw_call, sprite.target(), sprite.color(), texture_index);

        self.draw_calls.push(draw_call);
    }

    pub fn draw_sprite(&mut self, sprite: &Sprite) {
        if self.draw_calls.is_empty() {
            self.add_new_draw_call(sprite, 0);
        } else if self.draw_calls.last().unwrap().current_texture != sprite.texture() as _ {
            let dc = self.draw_calls.last_mut().unwrap();

            if dc.textures.len() >= self.capacity as usize {
                self.add_new_draw_call(sprite, 0);
            } else {
                match dc.textures.iter().position(|texture| texture.id() == sprite.texture().id()) {
                    Some(index) => {
                        Self::add_quad(dc, sprite.target(), sprite.color(), index as u32);
                    },
                    None => {
                        dc.textures.push(sprite.clone_texture_rc());
                        Self::add_quad(dc, sprite.target(), sprite.color(), dc.textures.len() as u32 - 1);
                    },
                }
            }
        } else {
            Self::add_quad(self.draw_calls.last_mut().unwrap(), sprite.target(), sprite.color(), 0);
        }
    }

    pub fn draw_calls(&self) -> usize {
        self.draw_calls.len()
    }
}

//                                ,;.
//                              .;  `'-;.-
//                            ,;`       `.,
//                          ._;           ;`
//                          `;            ;`
//                           ;_ .-.-,`;   `;
//                            ;`` '  o '`&`
//                             ` o\      ;
//                              `. `_   ;
//                           *     .~_.'  *
//                            )__.-\   |\ )
//                          .' \   |   | `,.
//                         ,'   `. `.  |  | `\
//                        /      `. |  | ,'   `\
//                      ,'    ,1  `.`. | |  ,   `\
//                     /     / L   `.| |,' ;_     `.
//                     \     \  \   `| |; ,' \    _)
//                      `.    `)~\   |-|  |   `.-' ~-.
//                        `._./   ;  | |  |  ,'      |
//                         (    ,'`; |-|  |  \   _.--<
//                          `-.'_,'  | |  |   `-(    7
//                            ;      ; `. |      >-Y-')
//                           /      ;   |  \     `-=p~
//                         ,'      ;    :  `.      `|.
//                        /       ;   |  `.  `.     ||
//                      ,'       /    |   ;.  `.    ||
//                     /       ,'    ,'  ' ;.   `.   ||
//                   ,'       /     '|    ';`-.   `. ||
//                  /        ;|`-._.,'-.__.'  `-.   `||
//                ,'       ,' P     |     q     `-.  ||.
//              ,'      ,-'  ;      ;     )        `-|| `-.
//           ,-'      ,'     ;     ,^.    d         || `~-.`-.
//         ,'      ,-'       `L   d  :    p         ||     `-.`.
//       ,'     ,-'        ___|   q__:    b........||_        ~.\
//     ,'   _,-'     __,--~   |    \ (_    \       |  `~~--._    `\
//    /   ,'    _,--~         (_   |   `\.__|      ;         `~-.._|
//   /  ,'   ,-~                \   \                              `
//  / ,'  ,-~    kg              `-._>
// | ; ,-~
// |/_/

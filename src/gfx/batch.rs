use std::rc::Rc;

use crate::{
    errors::BatchError,
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

#[derive(Debug)]
struct DrawCall {
    pub vb: Option<raw::VertexBuffer>,
    pub ib: Option<raw::IndexBuffer>,

    pub texture: Option<Rc<raw::Texture>>,

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

            texture: None,

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
}

impl Default for Batch {
    fn default() -> Self {
        let shader_source = include_str_from_root!("shaders/batch.wgsl");

        let pipeline = raw::Pipeline::new(&raw::PipelineConfig {
            shader_source,
            vertex_buffer_layouts: &[Vertex::desc()],
            bind_group_layouts: &[
                (0, &raw::Uniform::bind_group_layout(raw::ShaderStages::Vertex)),
                (1, &raw::Texture::bind_group_layout()),
            ],
            depth_stencil_write_enabled: false,
        });

        //

        let view = glam::Mat4::look_at_rh(glam::Vec3::new(0.0, 0.0, 1.0), glam::Vec3::new(0.0, 0.0, 0.0), glam::Vec3::Y);
        let proj = glam::Mat4::orthographic_rh(0.0, 1280.0, 0.0, 720.0, 0.0, 1.0);

        let camera_uniform = raw::Uniform::new(crate::cast(&(proj * view).to_cols_array()), raw::ShaderStages::Vertex);

        Self {
            pipeline,
            camera_uniform,

            draw_calls: vec![],

            is_drawing: false,
        }
    }
}

impl Batch {
    pub fn new() -> Self {
        Self::default()
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

                    rp.set_texture(1, draw_call.texture.as_ref().unwrap().as_ref());

                    rp.set_vertex_buffer(0, draw_call.vb.as_ref().unwrap());
                    rp.set_index_buffer(draw_call.ib.as_ref().unwrap());
                    rp.draw_indexed(0..draw_call.ib.as_ref().unwrap().len(), 0, 0..1);
                }

                Ok(())
            },
            None => Err(anyhow::anyhow!(BatchError::FrameIsNone)),
        }
    }

    fn add_quad(draw_call: &mut DrawCall, target: Rectangle<f32>, color: Color<f32>) {
        draw_call.vertex.extend_from_slice(&[
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

    pub fn draw_sprite(&mut self, sprite: &Sprite) {
        #[allow(clippy::all)] // clippy is sometimes delusional with its suggestions
        if self.draw_calls.is_empty() {
            let mut draw_call = DrawCall {
                texture: Some(sprite.clone_texture_rc()),
                ..Default::default()
            };

            draw_call.current_texture = sprite.texture() as _;
            Self::add_quad(&mut draw_call, sprite.target(), sprite.color());

            self.draw_calls.push(draw_call);
        } else if self.draw_calls.last().unwrap().current_texture != sprite.texture() as _ {
            let mut draw_call = DrawCall {
                texture: Some(sprite.clone_texture_rc()),
                ..Default::default()
            };

            draw_call.current_texture = sprite.texture() as _;
            Self::add_quad(&mut draw_call, sprite.target(), sprite.color());

            self.draw_calls.push(draw_call);
        } else {
            Self::add_quad(&mut self.draw_calls.last_mut().unwrap(), sprite.target(), sprite.color());
        }
    }
}

//
//
//
//
//

// I like little prince, its great metaphor on how life is with various people/challenges you can meet in your life.
// At least its how I understand that book.

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

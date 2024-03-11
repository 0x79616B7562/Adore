pub use wgpu;

//

pub mod bind;
pub mod config;
pub(crate) mod context;
mod depth;
mod dynamic_index_buffer;
mod dynamic_uniform;
mod dynamic_vertex_buffer;
mod frame;
mod index_buffer;
mod pipeline;
mod render_pass;
mod texture;
mod uniform;
mod vertex_buffer;

pub use config::*;
pub use context::{
    device,
    features,
    format,
    frame,
    init,
    queue,
    render,
    reset,
};
pub use depth::Depth;
pub use dynamic_index_buffer::DynamicIndexBuffer;
use dynamic_uniform::reset_dynamic_uniforms;
pub use dynamic_uniform::DynamicUniform;
pub use dynamic_vertex_buffer::DynamicVertexBuffer;
pub use frame::Frame;
pub use index_buffer::IndexBuffer;
pub use pipeline::Pipeline;
pub use render_pass::RenderPass;
pub use texture::Texture;
pub use uniform::*;
pub use vertex_buffer::VertexBuffer;

// ____________________________
// < What are you doing here? >
//  --------------------------
//         \
//          \
//             _~^~^~_
//         \) /  o o  \ (/
//           '_   -   _'
//           / '-----' \

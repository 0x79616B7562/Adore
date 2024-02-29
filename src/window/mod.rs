mod input;
#[allow(clippy::all)]
mod window;

pub use input::*;
pub use window::WindowConfig;
#[allow(unused_imports)]
pub(crate) use window::{
    abort,
    input,
    input_mut,
    raw,
    Window,
};

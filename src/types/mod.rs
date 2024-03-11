#![allow(dead_code)]

pub use memoffset::{
    offset_of,
    span_of,
};

mod cast;
mod color;
mod delta;
pub mod errors;
mod position;
mod rectangle;
mod size;

pub use cast::cast;
pub use color::Color;
pub use delta::Delta;
pub use position::Position;
pub use rectangle::Rectangle;
pub use size::Size;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Default)]
pub struct Rectangle<T>
where T: num::Num {
    pub x: T,
    pub y: T,
    pub width: T,
    pub height: T,
}

impl<T> Rectangle<T>
where T: num::Num
{
    pub fn new(x: T, y: T, width: T, height: T) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }
}

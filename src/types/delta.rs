#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Delta<T>
where T: num::Num {
    pub x: T,
    pub y: T,
}

impl<T> Delta<T>
where T: num::Num
{
    pub fn new(x: T, y: T) -> Self {
        Self {
            x,
            y,
        }
    }
}

impl<T> From<Delta<T>> for (T, T)
where T: num::Num
{
    fn from(val: Delta<T>) -> Self {
        (val.x, val.y)
    }
}

impl<T> From<(T, T)> for Delta<T>
where T: num::Num
{
    fn from(val: (T, T)) -> Self {
        Delta {
            x: val.0,
            y: val.1,
        }
    }
}

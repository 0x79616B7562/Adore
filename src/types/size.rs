#[repr(C)]
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Size<T>
where T: num::Num {
    pub width: T,
    pub height: T,
}

impl<T> From<Size<T>> for (T, T)
where T: num::Num
{
    fn from(val: Size<T>) -> Self {
        (val.width, val.height)
    }
}

impl<T> From<(T, T)> for Size<T>
where T: num::Num
{
    fn from(val: (T, T)) -> Self {
        Size {
            width: val.0,
            height: val.1,
        }
    }
}

impl<T> From<Size<T>> for [T; 2]
where T: num::Num
{
    fn from(val: Size<T>) -> Self {
        [val.width, val.height]
    }
}

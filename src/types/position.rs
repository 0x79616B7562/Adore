#[repr(C)]
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Position<T>
where T: num::Num {
    pub x: T,
    pub y: T,
}

impl<T> From<Position<T>> for (T, T)
where T: num::Num
{
    fn from(val: Position<T>) -> Self {
        (val.x, val.y)
    }
}

impl<T> From<(T, T)> for Position<T>
where T: num::Num
{
    fn from(val: (T, T)) -> Self {
        Position {
            x: val.0,
            y: val.1,
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color<T>
where T: num::Num + num::FromPrimitive {
    pub r: T,
    pub g: T,
    pub b: T,
    pub a: T,
}

impl<T> Default for Color<T>
where T: num::Num + num::FromPrimitive
{
    fn default() -> Self {
        Self {
            r: num::FromPrimitive::from_u8(1).unwrap(),
            g: num::FromPrimitive::from_u8(1).unwrap(),
            b: num::FromPrimitive::from_u8(1).unwrap(),
            a: num::FromPrimitive::from_u8(1).unwrap(),
        }
    }
}

impl<T> Color<T>
where T: num::Num + num::FromPrimitive
{
    pub fn new(r: T, g: T, b: T, a: T) -> Self {
        Self {
            r,
            b,
            g,
            a,
        }
    }
}

impl<T> From<Color<T>> for [T; 4]
where T: num::Num + num::FromPrimitive
{
    fn from(val: Color<T>) -> Self {
        [val.r, val.g, val.b, val.a]
    }
}

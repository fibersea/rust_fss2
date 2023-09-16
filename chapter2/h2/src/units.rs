/*! Contains list of units to make code more idiomatic and clear to use
 * 
 */


#[derive(Copy, Clone, Debug)]
pub struct Pixel {
    pub x: usize,
    pub y: usize,
}
impl Pixel {
    pub fn new(x: usize, y: usize) -> Self {
        Self {x, y}
    }
}

/// Represents size bounds for a visual square (i.e. for image size)
/// 
#[derive(Copy, Clone, Debug)]
pub struct SquareBoundsPx {
    pub width: usize,
    pub height: usize,
}

impl From<(usize, usize)> for SquareBoundsPx {
    fn from(value: (usize, usize)) -> SquareBoundsPx {
        SquareBoundsPx {
            width: value.0,
            height: value.1,
        }
    }
}
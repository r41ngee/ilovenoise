#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rgba {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: Option<u8>,
}

impl Rgba {
    pub fn new(r: u8, g: u8, b: u8, a: Option<u8>) -> Self {
        Self { r, g, b, a }
    }
}

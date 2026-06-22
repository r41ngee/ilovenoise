#[derive(Debug, Clone, Copy)]
pub struct Rgba {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: Option<u8>,
}

impl Rgba {
    #[allow(unused)]
    pub fn as_bytes(&self) -> [u8; 4] {
        let mut result = [255u8; 4];
        (result[0], result[1], result[2]) = (self.r, self.g, self.b);
        if let Some(a) = self.a {
            result[3] = a;
        }
        result
    }

    pub fn new(r: u8, g: u8, b: u8, a: Option<u8>) -> Self {
        Self {
            r,
            g,
            b,
            a
        }
    }
}
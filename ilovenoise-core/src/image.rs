use std::ops::{Index, IndexMut};

#[derive(Debug, Clone)]
pub struct Image {
    pub size: (u32, u32),
    pub pixels: Vec<crate::util::Rgba>,
}

impl Image {
    pub fn new(size: (u32, u32)) -> Self {
        Self {
            size,
            pixels: vec![crate::util::Rgba::new(0, 0, 0, None); (size.0 * size.1) as usize],
        }
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let mut data = vec![0; self.size.0 as usize * self.size.1 as usize * 4];

        #[allow(clippy::useless_conversion)]
        for (index, pixel) in data.chunks_mut(4).into_iter().enumerate() {
            pixel[0] = self.pixels[index].r;
            pixel[1] = self.pixels[index].g;
            pixel[2] = self.pixels[index].b;
            pixel[3] = self.pixels[index].a.unwrap_or(255);
        }

        data
    }
}

impl Index<(u32, u32)> for Image {
    type Output = crate::util::Rgba;

    fn index(&self, index: (u32, u32)) -> &Self::Output {
        let (x, y) = index;
        let idx = (y * self.size.0 + x) as usize;
        &self.pixels[idx]
    }
}

impl IndexMut<(u32, u32)> for Image {
    fn index_mut(&mut self, index: (u32, u32)) -> &mut Self::Output {
        let (x, y) = index;
        let idx = (y * self.size.0 + x) as usize;
        &mut self.pixels[idx]
    }
}

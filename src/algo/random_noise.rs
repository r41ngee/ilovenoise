use rand::RngExt;
use rand_chacha::ChaCha8Rng;
use super::Aglorithm;

pub struct RandomNoise {
    rng_thr: ChaCha8Rng
}

impl RandomNoise {
    pub fn new(rng_thr: ChaCha8Rng) -> Self {
        Self {
            rng_thr
        }
    }
}

impl Aglorithm for RandomNoise {
    fn draw(&mut self, image: &mut crate::image::Image) {
        for pixel in &mut image.pixels {
            let multiplier = (self.rng_thr.random_range(0f32..1f32) * 255f32) as u8;
            let (r, g, b) = (multiplier, multiplier, multiplier);
            let a = Some(255);
            *pixel = crate::util::Rgba::new(r, g, b, a);
        }
    }
}
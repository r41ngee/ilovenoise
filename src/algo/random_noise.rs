use super::Aglorithm;
use rand::RngExt;
use rand_chacha::ChaCha8Rng;

use rayon::prelude::*;
use rand::SeedableRng;

pub struct RandomNoise {
    rng_thr: ChaCha8Rng,
}

impl RandomNoise {
    pub fn new(rng_thr: ChaCha8Rng) -> Self {
        Self { rng_thr }
    }
}

impl Aglorithm for RandomNoise {
    fn draw(&mut self, image: &mut crate::image::Image) {
        let w = image.size.0 as usize;
        let seed: u64 = self.rng_thr.random_range(u64::MIN..=u64::MAX);


        image.pixels
        .par_chunks_mut(w)
        .enumerate()
        .for_each(|(py, row)| {
            let mut row_rng = ChaCha8Rng::seed_from_u64(
                seed ^ (py as u64).wrapping_mul(0x9e3779b97f4a7c15)
            );
            for pixel in row.iter_mut() {
                let val = (row_rng.random_range(0f32..1f32) * 255f32) as u8;
                *pixel = crate::util::Rgba::new(val, val, val, Some(255));
            }
        });
    }
}

#[cfg(test)]
mod tests {

    use crate::image::Image;
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;
    use std::ops::Range;

    use super::*;

    const DEFAULT_SIZE: (u32, u32) = (16, 16);
    const DEFAULT_SEED: u64 = 64;

    fn in_range<T: PartialOrd>(val: T, range: Range<T>) -> bool {
        val <= range.end && val >= range.start
    }

    #[test]
    fn random_clamp() {
        let rng = ChaCha8Rng::seed_from_u64(DEFAULT_SEED);
        let mut randnoise = RandomNoise::new(rng);
        let mut image = Image::new(DEFAULT_SIZE);

        randnoise.draw(&mut image);

        for p in image.pixels {
            assert!(
                in_range(p.r, 0u8..255u8) && in_range(p.g, 0u8..255u8) && in_range(p.b, 0u8..255u8)
            )
        }
    }

    #[test]
    fn random_determined() {
        let mut result: [crate::image::Image; 2] = [
            crate::image::Image::new(DEFAULT_SIZE),
            crate::image::Image::new(DEFAULT_SIZE),
        ];
        {
            let rng = ChaCha8Rng::seed_from_u64(DEFAULT_SEED);
            let mut randnoise = RandomNoise::new(rng);
            randnoise.draw(&mut result[0]);
        }

        {
            let rng = ChaCha8Rng::seed_from_u64(DEFAULT_SEED);
            let mut randnoise = RandomNoise::new(rng);
            randnoise.draw(&mut result[1]);
        }

        assert_eq!(result[0].pixels, result[1].pixels);
    }

    #[test]
    fn random_snapshot() {
        let rng = ChaCha8Rng::seed_from_u64(42);
        let mut randnoise = RandomNoise::new(rng);
        let mut image = crate::image::Image::new(DEFAULT_SIZE);
        randnoise.draw(&mut image);
        insta::assert_debug_snapshot!(image.pixels);
    }
}

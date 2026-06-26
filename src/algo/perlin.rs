use std::ops::{Index, IndexMut};

use rand::RngExt;
use rand_chacha::ChaCha8Rng;
use rayon::prelude::*;

pub struct Perlin {
    rng_thr: ChaCha8Rng,
    octaves: u32,
    persistence: f32,
    lacunarity: f64,
    size: (u32, u32),
    frequency: u32,
}

impl Perlin {
    pub fn new(
        size: (u32, u32),
        rng_thr: ChaCha8Rng,
        octaves: Option<u32>,
        persistence: Option<f32>,
        lacunarity: Option<f64>,
    ) -> Self {
        const DEF_OCTAVES: u32 = 4;
        const DEF_PERSISTENCE: f32 = 0.5;
        const DEF_LACUNARITY: f64 = 2.0;

        Self {
            rng_thr,
            size,
            octaves: octaves.unwrap_or(DEF_OCTAVES),
            persistence: persistence.unwrap_or(DEF_PERSISTENCE),
            lacunarity: lacunarity.unwrap_or(DEF_LACUNARITY),
            frequency: 8,
        }
    }
}

impl super::Aglorithm for Perlin {
    fn draw(&mut self, image: &mut crate::image::Image) {
        let grid_w = self.size.0 / self.frequency + 1;
        let grid_h = self.size.1 / self.frequency + 1;
        let lattice = Lattice::generate((grid_w, grid_h), &mut self.rng_thr);

        let octaves = self.octaves;
        let frequency = self.frequency;
        let persistence = self.persistence;
        let lacunarity = self.lacunarity;

        let w = image.size.0 as usize;

        image.pixels
            .par_chunks_mut(w)
            .enumerate()
            .for_each(|(py, row)| {
                for (px, pixel) in row.iter_mut().enumerate() {
                    let mut value = 0.0f32;
                    let mut amplitude = 1.0f32;
                    let mut freq = 1.0f64;
                    let mut max_value = 0.0f32;

                    for _ in 0..octaves {
                        let fx = px as f64 / frequency as f64 * freq;
                        let fy = py as f64 / frequency as f64 * freq;

                        let i = fx.floor() as i64;
                        let j = fy.floor() as i64;
                        let u = fx - i as f64;
                        let v = fy - j as f64;

                        let su = u * u * (3.0 - 2.0 * u);
                        let sv = v * v * (3.0 - 2.0 * v);

                        let g00 = lattice.get_wrapped(i, j);
                        let g10 = lattice.get_wrapped(i + 1, j);
                        let g01 = lattice.get_wrapped(i, j + 1);
                        let g11 = lattice.get_wrapped(i + 1, j + 1);

                        let n00 = g00.x * u as f32 + g00.y * v as f32;
                        let n10 = g10.x * (u - 1.0) as f32 + g10.y * v as f32;
                        let n01 = g01.x * u as f32 + g01.y * (v - 1.0) as f32;
                        let n11 = g11.x * (u - 1.0) as f32 + g11.y * (v - 1.0) as f32;

                        let top = n00 + (n10 - n00) * su as f32;
                        let bot = n01 + (n11 - n01) * su as f32;
                        let octave_val = top + (bot - top) * sv as f32;

                        value += octave_val * amplitude;
                        max_value += amplitude;
                        amplitude *= persistence;
                        freq *= lacunarity;
                    }

                    let n = ((value / max_value + 1.0) * 0.5 * 255.0) as u8;
                    *pixel = crate::util::Rgba::new(n, n, n, Some(255));
                }
            });
    }
}


#[derive(Debug, Clone)]
struct Lattice {
    data: Vec<Vector>,
    size: (u32, u32),
}

impl Index<(usize, usize)> for Lattice {
    type Output = Vector;
    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        &self.data[y * self.size.0 as usize + x]
    }
}

impl IndexMut<(usize, usize)> for Lattice {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
        &mut self.data[y * self.size.0 as usize + x]
    }
}

impl Lattice {
    fn new(size: (u32, u32)) -> Self {
        Self {
            size,
            data: vec![Vector::default(); (size.0 * size.1) as usize],
        }
    }

    fn generate(size: (u32, u32), rng: &mut ChaCha8Rng) -> Self {
        let mut lattice = Self::new(size);
        for v in &mut lattice.data {
            v.randomize(rng);
        }
        lattice
    }

    fn get_wrapped(&self, x: i64, y: i64) -> &Vector {
        let mx = x.rem_euclid(self.size.0 as i64) as usize;
        let my = y.rem_euclid(self.size.1 as i64) as usize;
        &self.data[my * self.size.0 as usize + mx]
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Vector {
    x: f32,
    y: f32,
}

impl Default for Vector {
    fn default() -> Self {
        Self { x: 0.0, y: 0.0 }
    }
}

impl Vector {
    fn randomize(&mut self, rng: &mut ChaCha8Rng) {
        let angle: f32 = rng.random_range(0.0..std::f32::consts::PI * 2.0);
        self.x = angle.cos();
        self.y = angle.sin();
    }
}

#[cfg(test)]
mod tests {
    use std::ops::Range;

    use super::*;
    use crate::algo::Aglorithm;
    use rand::SeedableRng;

    const DEFAULT_SEED: u64 = 42u64;
    const DEFAULT_SIZE: (u32, u32) = (16, 16);

    fn in_range<T: PartialOrd>(val: T, range: Range<T>) -> bool {
        val <= range.end && val >= range.start
    }

    #[test]
    fn perlin_clamp() {
        let rng = ChaCha8Rng::seed_from_u64(DEFAULT_SEED);
        let mut perlin = Perlin::new(DEFAULT_SIZE, rng, Some(2), None, None);
        let mut image = crate::image::Image::new(DEFAULT_SIZE);

        perlin.draw(&mut image);

        for p in image.pixels {
            assert!(
                in_range(p.r, 0u8..255u8) && in_range(p.g, 0u8..255u8) && in_range(p.b, 0u8..255u8)
            )
        }
    }

    #[test]
    fn perlin_determined() {
        let mut result: [crate::image::Image; 2] = [
            crate::image::Image::new(DEFAULT_SIZE),
            crate::image::Image::new(DEFAULT_SIZE),
        ];
        {
            let rng = ChaCha8Rng::seed_from_u64(DEFAULT_SEED);
            let mut perlin = Perlin::new(DEFAULT_SIZE, rng, Some(2), None, None);
            perlin.draw(&mut result[0]);
        }

        {
            let rng = ChaCha8Rng::seed_from_u64(DEFAULT_SEED);
            let mut perlin = Perlin::new(DEFAULT_SIZE, rng, Some(2), None, None);
            perlin.draw(&mut result[1]);
        }

        assert_eq!(result[0].pixels, result[1].pixels);
    }

    #[test]
    fn lattice_wrapping() {
        let lattice = Lattice::new(DEFAULT_SIZE);
        assert_eq!(*lattice.get_wrapped(0, 0), *lattice.get_wrapped(4, 4),);
    }

    #[test]
    fn octaves_influence() {
        let mut result: [crate::image::Image; 2] = [
            crate::image::Image::new(DEFAULT_SIZE),
            crate::image::Image::new(DEFAULT_SIZE),
        ];
        {
            let rng = ChaCha8Rng::seed_from_u64(DEFAULT_SEED);
            let mut perlin = Perlin::new(DEFAULT_SIZE, rng, Some(1), None, None);
            perlin.draw(&mut result[0]);
        }

        {
            let rng = ChaCha8Rng::seed_from_u64(DEFAULT_SEED);
            let mut perlin = Perlin::new(DEFAULT_SIZE, rng, Some(2), None, None);
            perlin.draw(&mut result[1]);
        }

        assert_ne!(result[0].pixels, result[1].pixels);
    }

    #[test]
    fn persistence_influence() {
        let mut result: [crate::image::Image; 2] = [
            crate::image::Image::new(DEFAULT_SIZE),
            crate::image::Image::new(DEFAULT_SIZE),
        ];
        {
            let rng = ChaCha8Rng::seed_from_u64(DEFAULT_SEED);
            let mut perlin = Perlin::new(DEFAULT_SIZE, rng, None, Some(0.5), None);
            perlin.draw(&mut result[0]);
        }

        {
            let rng = ChaCha8Rng::seed_from_u64(DEFAULT_SEED);
            let mut perlin = Perlin::new(DEFAULT_SIZE, rng, None, Some(0.7), None);
            perlin.draw(&mut result[1]);
        }

        assert_ne!(result[0].pixels, result[1].pixels);
    }

    #[test]
    fn lacunarity_influence() {
        let mut result: [crate::image::Image; 2] = [
            crate::image::Image::new(DEFAULT_SIZE),
            crate::image::Image::new(DEFAULT_SIZE),
        ];
        {
            let rng = ChaCha8Rng::seed_from_u64(DEFAULT_SEED);
            let mut perlin = Perlin::new(DEFAULT_SIZE, rng, None, None, Some(1.0));
            perlin.draw(&mut result[0]);
        }

        {
            let rng = ChaCha8Rng::seed_from_u64(DEFAULT_SEED);
            let mut perlin = Perlin::new(DEFAULT_SIZE, rng, None, None, Some(2.0));
            perlin.draw(&mut result[1]);
        }

        assert_ne!(result[0].pixels, result[1].pixels);
    }

    #[test]
    fn perlin_snapshot() {
        let rng = ChaCha8Rng::seed_from_u64(42);
        let mut perlin = Perlin::new((8, 8), rng, Some(2), None, None);
        let mut image = crate::image::Image::new(DEFAULT_SIZE);
        perlin.draw(&mut image);
        insta::assert_debug_snapshot!(image.pixels);
    }
}

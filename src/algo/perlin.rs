use std::{ops::{Index, IndexMut}, range::Range};

use rand::RngExt;
use rand_chacha::ChaCha8Rng;

pub struct Perlin {
    rng_thr: ChaCha8Rng,

    octaves: u32,
    persistence: f32,
    lacunarity: f64,

    size: (u32, u32),
    frequency: u32
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
            frequency: 8
        }
    }
}

impl super::Aglorithm for Perlin {
    fn draw(&mut self, image: &mut crate::image::Image) {
        let lattice = Lattice::generate((image.width as usize, image.height as usize), self.frequency, &mut self.rng_thr);
        

        todo!();
    }
}

#[derive(Debug, Clone)]
struct Lattice {
    data: Vec<Vector>,
    size: (u32, u32),
    frequency: u32
}

impl Index<(usize, usize)> for Lattice {
    type Output = Vector;
    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.data[index.1 * self.size.0 as usize + index.0]
    }
}

impl IndexMut<(usize, usize)> for Lattice {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.data[index.1 * self.size.0 as usize + index.0]
    }
}

impl Lattice {
    fn new(size: (usize, usize), frequency: u32) -> Self {
        Self {
            size: (size.0 as u32, size.1 as u32),
            data: vec![Vector::default(); (size.0 / frequency as usize) * (size.1 / frequency as usize)],
            frequency,
        }
    }

    pub fn generate(size: (usize, usize), frequency: u32, rand_thr:&mut ChaCha8Rng ) -> Self {
        let mut lattice = Self::new(size, frequency);
        for vector in &mut lattice.data {
            vector.randomize(rand_thr);
        }

        lattice
    }

    // pub fn point_range(&self, coord: (usize, usize)) -> Range<usize> {
        
    // }
}

#[derive(Debug, Clone, Copy)]
struct Vector {
    x: f32,
    y: f32
}

impl Default for Vector {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0
        }
    }
}

impl Vector {
    pub fn randomize(&mut self, rand_thr: &mut ChaCha8Rng) {
        self.x = rand_thr.random_range(-1.0..1.0);
        self.y = rand_thr.random_range(-1.0..1.0);
    }
}
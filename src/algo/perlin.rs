use rand_chacha::ChaCha8Rng;

pub struct Perlin {
    rng_thr: ChaCha8Rng,

    octaves: u32,
    persistence: f32,
    lacunarity: f64,
}

impl Perlin {
    pub fn new(
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
            octaves: octaves.unwrap_or(DEF_OCTAVES),
            persistence: persistence.unwrap_or(DEF_PERSISTENCE),
            lacunarity: lacunarity.unwrap_or(DEF_LACUNARITY)
        }
    }
}

impl super::Aglorithm for Perlin {
    fn draw(&mut self, image: &mut crate::image::Image) {
        todo!();
    }
}
pub mod perlin;
pub mod random_noise;

pub const ALGORITHMS: &[&str] = &["random", "perlin"];

pub trait Algorithm {
    fn draw(&mut self, image: &mut crate::image::Image);
}

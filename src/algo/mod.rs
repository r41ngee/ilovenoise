pub mod random_noise;

pub trait Aglorithm {
    fn draw(&mut self, image: &mut crate::image::Image);
}
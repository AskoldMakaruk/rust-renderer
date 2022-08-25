use std::io::Result;
use crate::renderer::scene::Scene;

pub(crate) mod console;
pub(crate) mod ppm_image;
pub(crate) mod obj_file;

pub(crate) trait Output {
    fn dump(&self, buff: &[f64], width: usize, height: usize) -> Result<()>;
}


pub(crate) trait Input {
   fn load(&self) -> Result<Scene>;
}
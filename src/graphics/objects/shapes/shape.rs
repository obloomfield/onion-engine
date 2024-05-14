
use crate::graphics::gl::BufferContents;

pub trait Shape {
    fn generate(param1: u16, param2: u16) -> Self;
    fn buffer(&self) -> BufferContents;
}
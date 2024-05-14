use winit::event::KeyEvent;

use crate::render::State;

pub trait App {
  fn resize(&mut self, state: &mut State, new_size: winit::dpi::PhysicalSize<u32>);
  fn input(&mut self, state: &mut State, event: &KeyEvent);
  fn update(&mut self, state: &mut State);
}
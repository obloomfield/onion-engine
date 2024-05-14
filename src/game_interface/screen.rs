use winit::event::KeyEvent;

use crate::{render::State, game_interface::app};

use app::App;

pub trait Screen<T: App> {
  fn resize(&mut self, game_state: &mut T, engine_state: &mut State, new_size: winit::dpi::PhysicalSize<u32>);
  fn input(&mut self, game_state: &mut T, engine_state: &mut State, event: &KeyEvent);
  fn update(&mut self, game_state: &mut T, engine_state: &mut State);
}
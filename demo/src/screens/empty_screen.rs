// To satisfy rust and its borrowing mechanics, need a screen with low instantiation impact...
// TODO: get better at rust.

use onion_engine::{
  game_interface::screen::Screen,
  render::State
};

use crate::gameapp::GameApp;

pub struct EmptyScreen {
}

impl EmptyScreen {
}

impl Screen<GameApp> for EmptyScreen {
    fn resize(&mut self, game_state: &mut GameApp, engine_state: &mut State, new_size: winit::dpi::PhysicalSize<u32>) {
        //
    }

    fn input(&mut self, game_state: &mut GameApp, engine_state: &mut State, event: &winit::event::KeyEvent) {
        //
    }

    fn update(&mut self, game_state: &mut GameApp, engine_state: &mut State) {
        //
    }
  }

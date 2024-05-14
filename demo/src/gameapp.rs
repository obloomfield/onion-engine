use onion_engine::{
  game_interface::{app::App, screen::Screen}, 
  render::State
};
use winit::event::KeyEvent;
use std::collections::HashMap;

use crate::screens::{demo_screen::DemoScreen, empty_screen::EmptyScreen};

pub struct GameApp {
  screens : HashMap<String, Box<dyn Screen<GameApp>>>,
  current_screen: String,
}
impl GameApp {
  pub fn init() -> Self {
    let mut screens: HashMap<String, Box<dyn Screen<GameApp>>> = HashMap::new();
    screens.insert("demo_screen".to_string(), Box::new(DemoScreen::init()));
    Self {
      screens,
      current_screen: "demo_screen".to_string(),
    }
  }
}

impl App for GameApp {

  fn resize(&mut self, state: &mut State, new_size: winit::dpi::PhysicalSize<u32>) {
    if let Some(screen) = self.screens.get_mut(&self.current_screen) {
      let mut screen = std::mem::replace(screen, Box::new(EmptyScreen{}));
      screen.resize(self, state, new_size);
      self.screens.insert(self.current_screen.clone(), screen);
    }
  }
  
  fn input(&mut self, state: &mut State, event: &KeyEvent) {
    if let Some(screen) = self.screens.get_mut(&self.current_screen) {
      let mut screen = std::mem::replace(screen, Box::new(EmptyScreen{}));
      screen.input(self, state, event);
      self.screens.insert(self.current_screen.clone(), screen);
    }
  }
  
  fn update(&mut self, state: &mut State) {
    if let Some(screen) = self.screens.get_mut(&self.current_screen) {
      let mut screen = std::mem::replace(screen, Box::new(EmptyScreen{}));
      screen.update(self, state);
      self.screens.insert(self.current_screen.clone(), screen);
    }
  }
}

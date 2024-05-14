use onion_engine::{
  game_interface::screen::Screen,
  render::State
};

use crate::gameapp::GameApp;
use crate::common::controls::CameraController;

pub struct DemoScreen {
  camera_controller: CameraController,
}

impl DemoScreen {
  pub fn init() -> Self {
    let camera_controller = CameraController::new(0.2);

    Self {
      camera_controller: camera_controller,
    }
  }
}

impl Screen<GameApp> for DemoScreen {
    fn resize(&mut self, game_state: &mut GameApp, engine_state: &mut State, new_size: winit::dpi::PhysicalSize<u32>) {
        // println!("resize!!");
    }

    fn input(&mut self, game_state: &mut GameApp, engine_state: &mut State, event: &winit::event::KeyEvent) {
        // println!("EVENT: {:?}", event);
        self.camera_controller.process_events(event);
    }

    fn update(&mut self, game_state: &mut GameApp, engine_state: &mut State) {
        self.camera_controller.update_camera(&mut engine_state.camera);
    }
}
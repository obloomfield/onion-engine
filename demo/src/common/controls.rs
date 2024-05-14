pub struct CameraController {
    speed: f32,
    is_forward_pressed: bool,
    is_backward_pressed: bool,
    is_left_pressed: bool,
    is_right_pressed: bool,
}

use winit::event::KeyEvent;
use winit::keyboard::KeyCode;
use winit::keyboard::PhysicalKey::Code;

use onion_engine::camera::Camera;

impl CameraController {
    pub fn new(speed: f32) -> Self {
        Self {
            speed,
            is_forward_pressed: false,
            is_backward_pressed: false,
            is_left_pressed: false,
            is_right_pressed: false,
        }
    }

    pub fn process_events(&mut self, event: &KeyEvent) {
        if let Code(keycode) = event.physical_key {
            let is_pressed = event.state.is_pressed();
            match keycode {
                KeyCode::KeyW | KeyCode::ArrowUp => {
                    self.is_forward_pressed = is_pressed;
                }
                KeyCode::KeyS | KeyCode::ArrowDown => {
                    self.is_backward_pressed = is_pressed;
                }
                KeyCode::KeyA | KeyCode::ArrowLeft => {
                    self.is_left_pressed = is_pressed;
                }
                KeyCode::KeyD | KeyCode::ArrowRight => {
                    self.is_right_pressed = is_pressed;
                }
                _ => {}
            }
        }
    }

    pub fn update_camera(&self, camera: &mut Camera) {
        let forward = camera.target - camera.eye;
        let forward_norm = forward.normalized();
        let forward_mag = forward.mag();

        // Prevents glitching when the camera gets too close to the
        // center of the scene.
        if self.is_forward_pressed && forward_mag > self.speed {
            camera.eye += forward_norm * self.speed;
        }
        if self.is_backward_pressed {
            camera.eye -= forward_norm * self.speed;
        }

        let right = forward_norm.cross(camera.up);

        // Redo radius calc in case the forward/backward is pressed.
        let forward = camera.target - camera.eye;
        let forward_mag = forward.mag();

        if self.is_right_pressed {
            // Rescale the distance between the target and the eye so
            // that it doesn't change. The eye, therefore, still
            // lies on the circle made by the target and eye.
            camera.eye = camera.target - (forward + right * self.speed).normalized() * forward_mag;
        }
        if self.is_left_pressed {
            camera.eye = camera.target - (forward - right * self.speed).normalized() * forward_mag;
        }
    }
}

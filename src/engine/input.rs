use winit::{event::{DeviceEvent, ElementState, KeyEvent, MouseButton, MouseScrollDelta, WindowEvent}, keyboard::{KeyCode, PhysicalKey}};

use super::Camera;

pub trait Input {
    fn process_keyboard(&mut self, _key: KeyCode, _state: ElementState) -> bool { false }
    fn process_mouse(&mut self, _mouse_dx: f64, _mouse_dy: f64) {}
    fn process_scroll(&mut self, _delta: &MouseScrollDelta) {}
    //fn process_keyboard(&self);
}
pub struct InputServer {
    camera_controller: CameraController,
    mouse_pressed: bool,
}

impl InputServer {
    pub const fn new() -> Self {
        Self { 
            mouse_pressed: false,
            camera_controller: CameraController::new(0.2, 2., std::f32::consts::PI / 300., 0.1)
        }
    }
    pub fn window_input(&mut self, event: &WindowEvent)-> bool {
        match event {
            WindowEvent::KeyboardInput { event: KeyEvent {
                physical_key: PhysicalKey::Code(key),
                state,
                ..
            }, ..} => self.camera_controller.process_keyboard(*key, *state),
            WindowEvent::MouseWheel { delta, .. } => {
                self.camera_controller.process_scroll(delta);
                true
            }
            WindowEvent::MouseInput {
                button: MouseButton::Left,
                state,
                ..
            } => {
                self.mouse_pressed = *state == ElementState::Pressed;
                true
            }
            _ => false
        }
    }

    pub fn device_input(&mut self, event: &DeviceEvent) {
        match event {
            DeviceEvent::MouseMotion { delta } => if self.mouse_pressed {
                self.camera_controller.process_mouse(delta.0, delta.1);
            },
            _ => ()
        }
    }

    pub fn update(&mut self, camera: &mut Camera) {
        self.camera_controller.update_camera(camera);
    }
}

pub struct CameraController {
    pub speed: f32, // meters
    pub angular_speed: f32, // rads
    pub zoom_speed: f32, // fraction
    theta: f32, // verticality
    phi: f32, // XY plane
    farness: f32, // meters
    forward_pressed: bool,
    backward_pressed: bool,
    left_pressed: bool,
    right_pressed: bool,
}

impl CameraController {
    pub const fn new(speed: f32, farness: f32, angular_speed: f32, zoom_speed: f32) -> Self {
        Self {
            speed,
            angular_speed,
            zoom_speed,
            theta: std::f32::consts::FRAC_PI_2,
            phi: 0.,
            farness,
            forward_pressed: false,
            backward_pressed: false,
            left_pressed: false,
            right_pressed: false,
        }
    }

    pub fn update_camera(&self, camera: &mut Camera) {
        let mut looking_at = camera.target - camera.eye;
        looking_at.z = 0.; // UP = 0

        let move_to = looking_at * (Into::<f32>::into(self.forward_pressed) - Into::<f32>::into(self.backward_pressed))
            + looking_at.cross(glam::Vec3::Z) * (Into::<f32>::into(self.left_pressed) - Into::<f32>::into(self.right_pressed));
        let norm = move_to.normalize();
        
        let move_to = if norm.is_nan() { move_to } else { norm * self.speed }; // cross should be fine because they're orthogonal
        camera.target += move_to;

        let theta_cos = self.theta.cos();
        let theta_sin = self.theta.sin();
        let phi_cos = self.phi.cos();
        let phi_sin = self.phi.sin();

        camera.eye = (
            camera.target.x + theta_sin * phi_cos * self.farness,
            camera.target.y + theta_sin * phi_sin * self.farness,
            camera.target.z + theta_cos * self.farness,
        ).into();

        camera.up = glam::Vec3::new(
            - phi_cos * theta_cos,
            - phi_sin * theta_cos,
            theta_sin,
        ).normalize();

        /*
        if (self.moving.fb === 0 && self.moving.lr === 0) return;
        
        self.target.x += self.speed * (
          - Math.cos(self.phi) * self.moving.fb
          - Math.sin(self.phi) * self.moving.lr
        );
        self.target.y += self.speed * (
          - Math.sin(self.phi) * self.moving.fb
          + Math.cos(self.phi) * self.moving.lr
        );
        updateModel.change = true; 
        
          [
            self.target.x + Math.sin(self.theta) * Math.cos(self.phi) * self.farness,
            self.target.y + Math.sin(self.theta) * Math.sin(self.phi) * self.farness,
            self.target.z + Math.cos(self.theta) * self.farness,
          ],
          [self.target.x, self.target.y, self.target.z],
          [
            - Math.cos(self.phi) * Math.cos(self.theta),
            - Math.sin(self.phi) * Math.cos(self.theta),
            Math.sin(self.theta),
          ]
          */
/*
        // Prevents glitching when the camera gets too close to the
        // center of the scene.
        if self.forward_pressed && forward_len > self.speed {
            camera.eye += forward_norm * self.speed;
        }
        if self.backward_pressed {
            camera.eye -= forward_norm * self.speed;
        }

        // Redo radius calc in case the forward/backward is pressed.
        let forward = camera.target - camera.eye;

        if self.right_pressed {
            // Rescale the distance between the target and the eye so 
            // that it doesn't change. The eye, therefore, still 
            // lies on the circle made by the target and eye.
            camera.eye = camera.target - ( glam::Quat::from_axis_angle(camera.up, -self.angular_speed) * forward );
        }
        if self.left_pressed {
            camera.eye = camera.target - (glam::Quat::from_axis_angle(camera.up,self.angular_speed) * forward);
        } */
    }
}

const PIXEL_TO_LINE_FRAC: f32 = 1. / 10.;
impl Input for CameraController {
    fn process_keyboard(&mut self, key: KeyCode, state: ElementState) -> bool {
        let is_pressed = state == ElementState::Pressed;
        match key {
            KeyCode::KeyW | KeyCode::ArrowUp => {
                self.forward_pressed = is_pressed;
                true
            }
            KeyCode::KeyA | KeyCode::ArrowLeft => {
                self.left_pressed = is_pressed;
                true
            }
            KeyCode::KeyS | KeyCode::ArrowDown => {
                self.backward_pressed = is_pressed;
                true
            }
            KeyCode::KeyD | KeyCode::ArrowRight => {
                self.right_pressed = is_pressed;
                true
            }
            _ => false,
        }
    }

    fn process_mouse(&mut self, mouse_dx: f64, mouse_dy: f64) {
        self.phi += mouse_dx as f32 * self.angular_speed;
        self.theta -= mouse_dy as f32 * self.angular_speed;

        if self.theta > std::f32::consts::PI {
            self.theta = std::f32::consts::PI;
        } else if self.theta < 0. {
            self.theta = 0.;
        }
    }

    fn process_scroll(&mut self, delta: &winit::event::MouseScrollDelta) {
        match *delta {
            MouseScrollDelta::LineDelta(_dx, dy) => {
                let zoom_factor = 1. + (dy.abs() * self.zoom_speed);
                let zoom_factor = if dy.is_sign_positive() { // Mouse wheel up is positive
                    1. / zoom_factor
                } else {
                    zoom_factor
                };

                self.farness *= zoom_factor;
            }
            MouseScrollDelta::PixelDelta(physical_position) => {
                let dy = physical_position.cast::<f32>().y * PIXEL_TO_LINE_FRAC;
                let zoom_factor = 1. + (dy.abs() * self.zoom_speed);
                let zoom_factor = if dy.is_sign_positive() {
                    zoom_factor
                } else {
                    1. / zoom_factor
                };

                self.farness *= zoom_factor;
            }
        }
    }
}

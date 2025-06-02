use glam::{Mat2, Vec2, Vec3, Vec3Swizzles};
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
            camera_controller: CameraController::new(0.05, 2., std::f32::consts::PI / 300., 0.1)
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
    phi: f32, // XY plane, 0 means looking towards (-1, 0)
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

        let theta_cos = self.theta.cos();
        let theta_sin = self.theta.sin();
        let phi_cos = self.phi.cos();
        let phi_sin = self.phi.sin();

        let move_to = Vec2::new(
            Into::<f32>::into(self.backward_pressed) - Into::<f32>::into(self.forward_pressed),
            Into::<f32>::into(self.left_pressed) - Into::<f32>::into(self.right_pressed),
        );

        if  move_to.x != 0. || move_to.y != 0. {
            let norm = move_to.normalize();
            let rotation = Mat2::from_cols_array(&[phi_cos, phi_sin, -phi_sin, phi_cos]);
            camera.target = camera.target.with_xy(camera.target.xy() + rotation * norm * self.speed);
        } 

        camera.eye = (
            camera.target.x + theta_sin * phi_cos * self.farness,
            camera.target.y + theta_sin * phi_sin * self.farness,
            camera.target.z + theta_cos * self.farness,
        ).into();

        camera.up = Vec3::new(
            - phi_cos * theta_cos,
            - phi_sin * theta_cos,
            theta_sin,
        ).normalize();
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
                    1. / zoom_factor
                } else {
                    zoom_factor
                };

                self.farness *= zoom_factor;
            }
        }
    }
}

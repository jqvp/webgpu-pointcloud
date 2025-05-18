use std::f32::consts::PI;
use glam::{Vec3, Mat4};
pub struct Camera {
    pub eye: Vec3,
    pub target: Vec3,
    pub up: Vec3,
    pub aspect: f32,
    pub fovy: f32,
    pub znear: f32,
    pub zfar: f32,
}
impl Camera {
    fn build_view_projection_matrix(&self) -> Mat4 {
        Mat4::perspective_lh(
            self.fovy/360.0*PI, 
            self.aspect, 
            self.znear, 
            self.zfar
        ) * Mat4::look_at_lh(
            self.eye,
            self.target,
            self.up)
    }
}

// We need this for Rust to store our data correctly for the shaders
#[repr(C)]
// This is so we can store this in a buffer
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
    pub time: f32,
    pub width: f32,
    pub height: f32,
    pub pixels: f32,
    model: Mat4,
    view_proj: Mat4,
}

impl CameraUniform {
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            time: 0.,
            width,
            height,
            pixels: 10.,
            view_proj: Mat4::IDENTITY,
            model: Mat4::IDENTITY,
        }
    }

    pub fn update_view_proj(&mut self, camera: &Camera) {
        self.view_proj = camera.build_view_projection_matrix();
    }
}

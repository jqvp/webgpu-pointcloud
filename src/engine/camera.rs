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
    pub fn build_view_projection_matrix(&self) -> Mat4 {
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

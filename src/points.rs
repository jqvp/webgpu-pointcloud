use std::f32::consts::PI;
use fastrand::*;
use glam::{Vec3, Mat4};
use las::Vector;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    position: [f32; 3],
}
unsafe impl bytemuck::Pod for Vertex {}
unsafe impl bytemuck::Zeroable for Vertex {}

impl Vertex {
    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
            ],
        }
    }

    pub fn new(x: f32, y: f32, z: f32) -> Vertex {
        Vertex { position: [x, y, z]}
    }
}

impl From<las::Vector<f32>> for Vertex {
    fn from(vector: Vector<f32>) -> Self {
        Vertex::new(vector.x, vector.y, vector.z)
    }
}

pub const SQUARE: &[Vertex] = &[
    Vertex { position: [-0.5, -0.5, 0.,]},
    Vertex { position: [0.5, -0.5, 0.,]},
    Vertex { position: [-0.5, 0.5, 0.,]},
    Vertex { position: [-0.5, 0.5, 0.,]},
    Vertex { position: [0.5, -0.5, 0.,]},
    Vertex { position: [0.5, 0.5, 0.,]},
];

pub fn get_intensities(quantity: usize) -> Vec<f32> {
    let mut rng: Rng = Rng::with_seed(0);
    let mut intensities = Vec::<f32>::with_capacity(quantity);
    for _ in 0..quantity {
        intensities.push(rng.f32());
    }

    intensities
}

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
#[derive(Copy, Clone)]
pub struct CameraUniform {
    pub time: f32,
    pub width: f32,
    pub height: f32,
    pub pixels: f32,
    model: Mat4,
    view_proj: Mat4,
}
unsafe impl bytemuck::Pod for CameraUniform {}
unsafe impl bytemuck::Zeroable for CameraUniform {}

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

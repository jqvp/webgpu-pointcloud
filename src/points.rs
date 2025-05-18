
use glam::Vec3;
use fastrand::*;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    position: [f32; 3],
}

impl Vertex {
    #[allow(dead_code)]
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

    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3::new(x, y, z)
    }
}

pub fn get_intensities(quantity: usize) -> Vec<f32> {
    let mut rng: Rng = Rng::with_seed(0);
    let mut intensities = Vec::<f32>::with_capacity(quantity);
    for _ in 0..quantity {
        intensities.push(rng.f32());
    }

    intensities
}

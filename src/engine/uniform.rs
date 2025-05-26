use super::*;
use glam::Mat4;

// We need this for Rust to store our data correctly for the shaders
#[repr(C)]
// This is so we can store this in a buffer
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct UniformData {
    pub time: f32,
    pub width: f32,
    pub height: f32,
    pub pixels: f32,
    model: Mat4,
    view_proj: Mat4,
}

impl UniformData {
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

pub struct Uniform {
    pub camera_uniform: UniformData,
    uniform_buffer: wgpu::Buffer,
    uniform_bind_group: wgpu::BindGroup,
}

impl Uniform {
    pub fn new(
        queue: &wgpu::Queue, 
        device: &wgpu::Device, 
        uniform_bind_group_layout: &wgpu::BindGroupLayout, 
        camera: &Camera, 
        width: u32, 
        height: u32, 
    ) -> Self{
        let mut camera_uniform = UniformData::new(width as f32, height as f32);
        camera_uniform.update_view_proj(camera);

        let uniform_buffer = device.create_buffer(
            &wgpu::BufferDescriptor {
                label: Some("Uniform Buffer"),
                size: (std::mem::size_of::<UniformData>()) as u64,
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            }
        );
        queue.write_buffer(&uniform_buffer, 0, bytemuck::cast_slice(&[camera_uniform]));

        let uniform_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: uniform_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: uniform_buffer.as_entire_binding(),
                }
            ],
            label: Some("uniform_bind_group"),
        });

        Uniform { 
            camera_uniform, 
            uniform_buffer,
            uniform_bind_group
        }
    }

    pub fn update(&mut self, camera: &Camera, queue: &wgpu::Queue) {
        self.camera_uniform.update_view_proj(camera);
        queue.write_buffer(&self.uniform_buffer, 0, bytemuck::cast_slice(&[self.camera_uniform]));
    }
}

impl<'a> Encode<'a> for Uniform {
    fn record_command(&'a self, recorder: &mut impl RenderEncoder<'a>) {
        recorder.set_bind_group(0, Some(&self.uniform_bind_group), &[]);
    }
}

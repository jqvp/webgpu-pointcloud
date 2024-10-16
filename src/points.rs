use fastrand::*;

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

pub const SQUARE: &[Vertex] = &[
    Vertex { position: [-0.5, -0.5, 0.,]},
    Vertex { position: [0.5, -0.5, 0.,]},
    Vertex { position: [-0.5, 0.5, 0.,]},
    Vertex { position: [-0.5, 0.5, 0.,]},
    Vertex { position: [0.5, -0.5, 0.,]},
    Vertex { position: [0.5, 0.5, 0.,]},
];

pub fn get_points(quantity: usize) -> Vec<Vertex> {
    let mut rng: Rng = Rng::with_seed(10);
    let mut vertices = Vec::<Vertex>::with_capacity(quantity);
    for _ in 0..quantity {
        vertices.push(Vertex { position: [ rng.f32()-0.5, rng.f32()-0.5, rng.f32()-0.5] });
    }

    vertices
}

pub fn get_intensities(quantity: usize) -> Vec<f32> {
    let mut rng: Rng = Rng::with_seed(10);
    let mut intensities = Vec::<f32>::with_capacity(quantity);
    for _ in 0..quantity {
        intensities.push(rng.f32());
    }

    intensities
}

#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.5,
    0.0, 0.0, 0.0, 1.0,
);

pub struct Camera {
    pub eye: cgmath::Point3<f32>,
    pub target: cgmath::Point3<f32>,
    pub up: cgmath::Vector3<f32>,
    pub aspect: f32,
    pub fovy: f32,
    pub znear: f32,
    pub zfar: f32,
}
impl Camera {
    fn build_view_projection_matrix(&self) -> cgmath::Matrix4<f32> {
        // 1.
        let view = cgmath::Matrix4::look_at_rh(self.eye, self.target, self.up);
        // 2.
        let proj = cgmath::perspective(cgmath::Deg(self.fovy), self.aspect, self.znear, self.zfar);

        // 3.
        return OPENGL_TO_WGPU_MATRIX * proj * view;
    }
}

// We need this for Rust to store our data correctly for the shaders
#[repr(C)]
// This is so we can store this in a buffer
#[derive(Copy, Clone)]
pub struct CameraUniform {
    // We can't use cgmath with bytemuck directly, so we'll have
    // to convert the Matrix4 into a 4x4 f32 array
    pub time: f32,
    pub width: f32,
    pub height: f32,
    pub pixels: f32,
    model: [[f32; 4]; 4],
    view_proj: [[f32; 4]; 4],
}
unsafe impl bytemuck::Pod for CameraUniform {}
unsafe impl bytemuck::Zeroable for CameraUniform {}

impl CameraUniform {
    pub fn new(width: f32, height: f32) -> Self {
        use cgmath::SquareMatrix;
        Self {
            time: 0.,
            width,
            height,
            pixels: 10.,
            view_proj: cgmath::Matrix4::identity().into(),
            model: cgmath::Matrix4::identity().into(),
        }
    }

    pub fn update_view_proj(&mut self, camera: &Camera) {
        self.view_proj = camera.build_view_projection_matrix().into();
    }
}

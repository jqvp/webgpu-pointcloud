use glam::{Vec3, Mat4};

#[derive(Debug)]
pub struct Camera {
    pub eye: Vec3,
    pub target: Vec3,
    pub up: Vec3,
    pub projection: Projection,
}
impl Camera {
    pub fn build_view_projection_matrix(&self) -> Mat4 {
        self.projection.make_matrix() * Mat4::look_at_lh(
            self.eye,
            self.target,
            self.up)
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Projection {
    Perspective {
        near: f32,
        far: f32,
        aspect: f32,
        fovy: f32, // radians
    },
    Orthographic {
        near: f32,
        far: f32,
        width: f32, // world units
        height: f32,
    }
}

impl Projection {
    pub const fn perspective(near: f32, far: f32, width: f32, height: f32, fovy: f32) -> Self {
        Self::Perspective { near, far, aspect: width/height, fovy }
    }

    pub fn make_matrix(&self) -> Mat4 {
        match *self {
            Projection::Perspective { aspect, fovy, near, far } => 
                Mat4::perspective_lh(fovy, aspect, near, far),
            Projection::Orthographic { near, far, width, height } => 
                Mat4::orthographic_lh(-width*0.5, width*0.5, -height*0.5, height*0.5, near, far),
        }
    }

    pub fn to_orthographic(&mut self, distance: f32) {
        match *self {
            Projection::Perspective { near, far, aspect, fovy } => {
                let height  = distance * 2. / (fovy / 2.).tan(); // distance is cos, height/2 is sin, fovy/2 is the angle

                *self = Projection::Orthographic { near, far, width: aspect * height, height }
            },
            _ => (),
        }
    }

    pub fn to_perspective(&mut self, distance: f32) {
        match *self {
            Projection::Orthographic { near, far, width, height } =>
                *self = Projection::Perspective { near, far, aspect: width/height, fovy: 2. * (height/distance/2.).atan() },
            _ => (),
        }
    }

    pub fn resize(&mut self, old_height: f32, old_width: f32, new_height: f32, new_width: f32) {
        match *self {
            Projection::Perspective { near, far, fovy, .. } => 
                *self = Self::perspective(near, far, new_width, new_height, fovy),
            Projection::Orthographic { near, far, width, height } => {
                *self = Self::Orthographic { 
                    near, 
                    far, 
                    width: new_width / old_width * width,
                    height: new_height / old_height * height,
                }
            },
        }
    }
}
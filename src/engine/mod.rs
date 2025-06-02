mod engine;
mod camera;
mod input;
mod uniform;
mod pipeline;

pub use engine::Engine;
pub use camera::*;
pub use uniform::*;

use wgpu::util::RenderEncoder;

pub trait Encode<'a> {
    fn record_command(&'a self, recorder: &mut impl RenderEncoder<'a>);
}

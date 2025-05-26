use std::io::Cursor;
use glam::Vec3;
use las::Reader;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
#[cfg(target_arch = "wasm32")]
use web_sys::{Request, RequestInit, RequestMode, Response};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen_futures::JsFuture;

use crate::engine::Encode;


pub struct Pointcloud {
    points: Vec<Vec3>,
    intensities: Vec<f32>,
    point_buffer: wgpu::Buffer,
    intensity_buffer: wgpu::Buffer,
}

impl Pointcloud {
    pub async fn from_las(url: &str, device: &wgpu::Device, queue: &wgpu::Queue) -> Result<Pointcloud, Box<dyn std::error::Error>> {
        #[cfg(not(target_family = "wasm"))]
        let bytes = {
            reqwest::blocking::get(url)?.bytes()?
        };
        #[cfg(target_family = "wasm")]
        let bytes = {
            let opts = RequestInit::new();
            opts.set_method("GET");
            opts.set_mode(RequestMode::Cors);
            let req = Request::new_with_str_and_init(&url, &opts).unwrap();
            let window = web_sys::window().unwrap();
            let resp_value = JsFuture::from(window.fetch_with_request(&req)).await.unwrap();
        
            assert!(resp_value.is_instance_of::<Response>());
            
            let resp: Response = resp_value.dyn_into().unwrap();
            let array = JsFuture::from(resp.array_buffer().unwrap()).await.unwrap();
            js_sys::Uint8Array::new(&array).to_vec()
        };

        let len = bytes.len();
        let (points, intensities) = Pointcloud::read_points(Reader::new(Cursor::new(bytes)).unwrap(), len);

        let point_buffer = device.create_buffer(
            &wgpu::BufferDescriptor {
                label: Some("Point Buffer"),
                size: (points.len()*12) as u64,
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            }
        );
        queue.write_buffer(&point_buffer, 0, bytemuck::cast_slice(&points));

        let intensity_buffer = device.create_buffer(
            &wgpu::BufferDescriptor {
                label: Some("Intensity Buffer"),
                size: (intensities.len()*4) as u64,
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            }
        );
        queue.write_buffer(&intensity_buffer, 0, bytemuck::cast_slice(&intensities)); 

        Ok(Pointcloud { 
            points, 
            intensities, 
            point_buffer,
            intensity_buffer
        })
    }

    pub fn points(&self) -> &[Vec3] {
        &self.points
    }

    fn read_points(mut reader: Reader, len: usize) -> (Vec<Vec3>, Vec<f32>) {
        let mut points = Vec::with_capacity(len);
        let mut intensities = Vec::with_capacity(len);

        let x_half = (reader.header().bounds().max.x - reader.header().bounds().min.x) * 0.5;
        let y_half = (reader.header().bounds().max.y - reader.header().bounds().min.y) * 0.5;
        let z_half = (reader.header().bounds().max.z - reader.header().bounds().min.z) * 0.5;
        let transforms = reader.header().transforms().to_owned();
        let min = reader.header().bounds().min;

        for wrapped_point in reader.points() {
            let point = wrapped_point.unwrap();
            points.push(Vec3::new(
                ((point.x - min.x - x_half) * transforms.x.scale) as f32,
                ((point.y- min.y - y_half) * transforms.y.scale) as f32,
                ((point.z- min.z - z_half) * transforms.z.scale) as f32,
            ));
            intensities.push(point.intensity as f32 / 65535.);
        }
        
        (points, intensities)
    }

}

impl<'a> Encode<'a> for Pointcloud {
    fn record_command(&'a self, recorder: &mut impl wgpu::util::RenderEncoder<'a>) {
        
        recorder.set_vertex_buffer(0, self.point_buffer.slice(..));
        recorder.set_vertex_buffer(1, self.intensity_buffer.slice(..));
        recorder.draw(0..4, 0..self.points().len() as u32);
    }
}

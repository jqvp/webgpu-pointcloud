use std::io::Cursor;
use glam::{DVec3, Vec3};
use las::{Reader, Vector};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
#[cfg(target_arch = "wasm32")]
use web_sys::{Request, RequestInit, RequestMode, Response};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen_futures::JsFuture;

use crate::engine::Encode;

fn into_dvec3(v: Vector<f64>) -> DVec3 {
    DVec3 { x: v.x, y: v.y, z: v.z }
}
fn into_vec3(v: DVec3) -> Vec3 {
    Vec3 { x: v.x as f32, y: v.y as f32, z: v.z as f32 }
}

pub struct Pointcloud {
    points: Vec<Vec3>,
    intensities: Vec<f32>,
    point_buffer: wgpu::Buffer,
    intensity_buffer: wgpu::Buffer,
    //return_number: Vec<u8>,
    //number_of_returns: Vec<u8>,
    //classification: Vec<u8>,
}

impl Pointcloud {
    pub async fn from_las(device: &wgpu::Device, queue: &wgpu::Queue, url: &str) -> Result<Pointcloud, Box<dyn std::error::Error>> {
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
        let (points, intensities) = Pointcloud::read_las(Reader::new(Cursor::new(bytes)).unwrap(), len);

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

    fn read_las(mut reader: Reader, len: usize) -> (Vec<Vec3>, Vec<f32>) {
        let mut points = Vec::with_capacity(len);
        let mut intensities = Vec::with_capacity(len);
        
        let header = reader.header();

        let half = (into_dvec3(header.bounds().max) - into_dvec3(header.bounds().min)) * 0.5;
        let transforms = header.transforms().to_owned();
        let min = into_dvec3(header.bounds().min);

        for wrapped_point in reader.points() {
            let point = wrapped_point.unwrap();
            points.push(
                into_vec3( ( DVec3::new(point.x, point.y, point.z) - min - half)
                * DVec3::new(transforms.x.scale, transforms.y.scale, transforms.z.scale) )
            );
            
            intensities.push(point.intensity as f32);
        }

        (points, intensities)
    }

}

impl<'a> Encode<'a> for Pointcloud {
    fn record_command(&'a self, recorder: &mut impl wgpu::util::RenderEncoder<'a>) {
        
        recorder.set_vertex_buffer(0, self.point_buffer.slice(..));
        recorder.set_vertex_buffer(1, self.intensity_buffer.slice(..));
        recorder.draw(0..4, 0..self.points.len() as u32);
    }
}

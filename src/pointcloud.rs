use std::io::Cursor;
use glam::Vec3;
use las::Reader;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
#[cfg(target_arch = "wasm32")]
use web_sys::{Request, RequestInit, RequestMode, Response};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen_futures::JsFuture;

use crate::points::Vertex;


pub struct Pointcloud {
    points: Vec<Vec3>,
}

impl Pointcloud {
    pub async fn from_las(url: &str) -> Result<Pointcloud, Box<dyn std::error::Error>> {
        cfg_if::cfg_if! {
        if #[cfg(target_arch = "wasm32")] {
            let opts = RequestInit::new();
            opts.set_method("GET");
            opts.set_mode(RequestMode::Cors);
            let req = Request::new_with_str_and_init(&url, &opts).unwrap();
            let window = web_sys::window().unwrap();
            let resp_value = JsFuture::from(window.fetch_with_request(&req)).await.unwrap();
        
            assert!(resp_value.is_instance_of::<Response>());
            
            let resp: Response = resp_value.dyn_into().unwrap();
            let array = JsFuture::from(resp.array_buffer().unwrap()).await.unwrap();
            let las_vec = js_sys::Uint8Array::new(&array).to_vec();

            let points = Pointcloud::read_points(Reader::new(Cursor::new(las_vec)).unwrap());

            Ok(Pointcloud { points })
        } else {
            let body = reqwest::blocking::get(url)?.bytes()?;
            let points = Pointcloud::read_points(Reader::new(Cursor::new(body)).unwrap());

            Ok(Pointcloud { points })
        }}
    }

    pub fn points(&self) -> &[Vec3] {
        &self.points
    }

    fn read_points(mut reader: Reader) -> Vec<Vec3> {
        let mut points = Vec::new();

        let x_half = (reader.header().bounds().max.x - reader.header().bounds().min.x) * 0.5;
        let y_half = (reader.header().bounds().max.y - reader.header().bounds().min.y) * 0.5;
        let z_half = (reader.header().bounds().max.z - reader.header().bounds().min.z) * 0.5;
        let transforms = reader.header().transforms().to_owned();
        let min = reader.header().bounds().min;

        for wrapped_point in reader.points() {
            let point = wrapped_point.unwrap();
            points.push(Vertex::new(
                ((point.x - min.x - x_half) * transforms.x.scale) as f32,
                ((point.y- min.y - y_half) * transforms.y.scale) as f32,
                ((point.z- min.z - z_half) * transforms.z.scale) as f32,
            ));
        }
        
        points
    }
}

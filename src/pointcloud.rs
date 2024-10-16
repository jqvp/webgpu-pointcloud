use std::io::Cursor;

use cgmath::num_traits::ToPrimitive;
use las::Reader;
use crate::points::Vertex;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
#[cfg(target_arch = "wasm32")]
use web_sys::{Request, RequestInit, RequestMode, Response};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen_futures::JsFuture;


pub struct Pointcloud {
    points: Vec<Vertex>,
}

impl Pointcloud {
    pub async fn from_las(url: &str) -> Result<Pointcloud, Box<dyn std::error::Error>> {
        cfg_if::cfg_if! {
        if #[cfg(target_arch = "wasm32")] {
            let mut opts = RequestInit::new();
            opts.method("GET");
            opts.mode(RequestMode::Cors);
            let req = Request::new_with_str_and_init(&url, &opts).unwrap();
            let window = web_sys::window().unwrap();
            let resp_value = JsFuture::from(window.fetch_with_request(&req)).await.unwrap();
        
            // `resp_value` is a `Response` object.
            assert!(resp_value.is_instance_of::<Response>());
            let resp: Response = resp_value.dyn_into().unwrap();
            let array = JsFuture::from(resp.array_buffer().unwrap()).await.unwrap();
            let las_vec = js_sys::Uint8Array::new(&array).to_vec();

            let mut reader = Reader::new(Cursor::new(las_vec)).unwrap();
            let mut points = Vec::new();
            let x_half = (reader.header().bounds().max.x - reader.header().bounds().min.x).to_f32().unwrap() * 0.5;
            let y_half = (reader.header().bounds().max.y - reader.header().bounds().min.y).to_f32().unwrap() * 0.5;
            let z_half = (reader.header().bounds().max.z - reader.header().bounds().min.z).to_f32().unwrap() * 0.5;
            for wrapped_point in reader.points() {
                let point = wrapped_point.unwrap();
                points.push(Vertex::new(point.x.to_f32().unwrap() - x_half, point.y.to_f32().unwrap() - y_half, point.z.to_f32().unwrap() - z_half));
            }

            Ok(Pointcloud { points })
        } else {
            let body = reqwest::blocking::get(url)?.bytes()?;
            let mut reader = Reader::new(Cursor::new(body)).unwrap();
            let mut points = Vec::new();
            let x_half = (reader.header().bounds().max.x - reader.header().bounds().min.x).to_f32().unwrap() * 0.5;
            let y_half = (reader.header().bounds().max.y - reader.header().bounds().min.y).to_f32().unwrap() * 0.5;
            let z_half = (reader.header().bounds().max.z - reader.header().bounds().min.z).to_f32().unwrap() * 0.5;
            for wrapped_point in reader.points() {
                let point = wrapped_point.unwrap();
                points.push(Vertex::new(point.x.to_f32().unwrap() - x_half, point.y.to_f32().unwrap() - y_half, point.z.to_f32().unwrap() - z_half));
            }

            Ok(Pointcloud { points })
        }}
    }

    pub fn points(&self) -> &[Vertex] {
        &self.points
    }
}

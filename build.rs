use naga::{front::wgsl, valid::{Capabilities, ValidationFlags, Validator}};
use std::fs;

fn main() {
    println!("cargo::rerun-if-changed=src/shaders");
    fs::read_dir("src/shaders/").expect("src/shaders/ folder required")
        .filter(|x| x.as_ref().unwrap().metadata().unwrap().is_file())
        .map(|x| fs::read_to_string(x.unwrap().path()).unwrap())
        .for_each(|content| {        
            if let Err(e) = Validator::new(ValidationFlags::all(), Capabilities::all())
            .validate(&wgsl::parse_str(&content).unwrap()) {
                panic!("{}", e);
            }
        });
    
}
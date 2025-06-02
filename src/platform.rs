#![allow(dead_code)]

#[cfg(not(target_family = "wasm"))]
pub fn print(s: &String) {
    println!("{}", s);
}

#[cfg(target_family = "wasm")]
pub fn print(s: &String) {
    web_sys::console::log_1(&s.into());
}
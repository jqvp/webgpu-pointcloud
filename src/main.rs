use lib_wasm_webgpu_backend::run;

fn main() {
  pollster::block_on(run());
}

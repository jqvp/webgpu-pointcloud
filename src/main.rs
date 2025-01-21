use lib_webgpu_pointcloud::run;

fn main() {
  pollster::block_on(run());
}

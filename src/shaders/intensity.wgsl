const square = array(
    vec2f(-0.5, -0.5),
    vec2f(0.5, -0.5),
    vec2f(-0.5, 0.5),
    vec2f(0.5, 0.5),
);
const pi = 3.14159265359;
override circleEnabled = true;

fn hsl_to_rgb(hsl: vec3f) -> vec3f {
  var rgb: vec3f;

  if hsl.y == 0. {
    rgb = vec3(hsl.z); // achromatic
  } else {
    var q: f32;
    if hsl.z < 0.5 {
      q = hsl.z * (1. + hsl.y);
    } else {
      q = hsl.z + hsl.y - hsl.z * hsl.y;
    }
    let p = 2. * hsl.z - q;
    rgb.r = hue_to_rgb(p, q, hsl.x + 1./3);
    rgb.g = hue_to_rgb(p, q, hsl.x);
    rgb.b = hue_to_rgb(p, q, hsl.x - 1./3);
  }
  return rgb;
}
fn hue_to_rgb(p: f32, q: f32, t: f32) -> f32 {
  var t2 = fract(t);
  if t2 < 1./6 {return p + (q - p) * 6. * t2;}
  if t2 < 1./2 {return q;}
  if t2 < 2./3 {return p + (q - p) * (2./3 - t2) * 6.;}
  return p;
}
fn to_fragment_coords(x: f32, y: f32) -> vec2f {
  return vec2f((x+1) * 0.5 * unif.width, (-y+1) * 0.5 * unif.height);
}

struct VertexIn {
  @location(0) point: vec3f,
  @location(1) intensity: f32,
  @builtin(vertex_index) index: u32,
};
struct VertexOut {
  @builtin(position) position: vec4f,
  @location(0) point: vec2f,
  @location(1) intensity: f32,
};
struct FragOut {
  @location(0) color: vec4f,
};

struct Uniforms {
  time: f32,
  width: f32,
  height: f32,
  pixels: f32,
  view_matrix: mat4x4f,
  model_matrix: mat4x4f,
};
@group(0) @binding(0) var<uniform> unif: Uniforms;

@vertex fn vs_main(in: VertexIn) -> VertexOut {
  var point = unif.view_matrix * unif.model_matrix * vec4f(in.point, 1);
  point = point/point.w;
  let vert = point + vec4f(square[in.index] / vec2f(unif.width, unif.height) * unif.pixels * 2, 0, 0);

  return VertexOut(vert, to_fragment_coords(point.x, point.y), in.intensity);
} 

@fragment fn fs_main(in: VertexOut) -> FragOut {
  if circleEnabled && distance(in.position.xy, in.point) > unif.pixels*0.5 {
    discard;
  }

  let color = hsl_to_rgb(vec3f(
    in.intensity,
    1,
    0.5));
  return FragOut(vec4f(color, 1));
}

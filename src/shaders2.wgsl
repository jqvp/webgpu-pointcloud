const pi = 3.14159265359;
override circleEnabled = true;
fn hslToRgb(hsl: vec3f) -> vec3f {
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
    rgb.r = hueToRgb(p, q, hsl.x + 1./3);
    rgb.g = hueToRgb(p, q, hsl.x);
    rgb.b = hueToRgb(p, q, hsl.x - 1./3);
  }
  return rgb;
}
fn hueToRgb(p: f32, q: f32, t: f32) -> f32 {
  var t2 = fract(t);
  if t2 < 1./6 {return p + (q - p) * 6. * t2;}
  if t2 < 1./2 {return q;}
  if t2 < 2./3 {return p + (q - p) * (2./3 - t2) * 6.;}
  return p;
}
fn toFragmentCoords(x: f32, y: f32) -> vec2f {
  return vec2f((x+1) * 0.5 * unif.width, (-y+1) * 0.5 * unif.height);
}

struct VertexIn {
  @location(0) vert: vec3f,
  @location(1) point: vec3f,
  @location(2) intensity: f32,
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
  modelMatrix: mat4x4f,
  viewMatrix: mat4x4f,
};
@group(0) @binding(0) var<uniform> unif: Uniforms;

@vertex fn vs_main(in: VertexIn) -> VertexOut {
  var point = unif.viewMatrix * unif.modelMatrix * vec4f(in.point, 1);
  point = point/point.w;
  let vert = point + vec4f(in.vert.x * unif.pixels / unif.width * 2, in.vert.y * unif.pixels / unif.height * 2, 0, 0);

  return VertexOut(vert, toFragmentCoords(point.x, point.y), in.intensity);
} 

@fragment fn fs_main(in: VertexOut) -> FragOut {
  if circleEnabled && distance(in.position.xy, in.point) > unif.pixels*0.5 {
    discard;
  }

  let color = hslToRgb(vec3f(
    in.intensity,
    1,
    0.5));
  return FragOut(vec4f(color, 1));
}

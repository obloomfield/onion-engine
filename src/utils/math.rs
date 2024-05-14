use ultraviolet as uv;

pub fn lerp(a: f32, b: f32, t: f32) -> f32 {
  a + t * (b - a)
}

pub fn lerp_vec2(a: uv::Vec2, b: uv::Vec2, t: f32) -> uv::Vec2 {
  uv::Vec2::new(
    lerp(a.x, b.x, t),
    lerp(a.y, b.y, t)
  )
}

pub fn lerp_vec3(a: uv::Vec3, b: uv::Vec3, t: f32) -> uv::Vec3 {
  uv::Vec3::new(
    lerp(a.x, b.x, t),
    lerp(a.y, b.y, t),
    lerp(a.z, b.z, t)
  )
}

// Y-UP
pub fn cyl_to_cartesian(r: f32, theta: f32, y: f32) -> uv::Vec3 {
  uv::Vec3::new(
    r * theta.cos(),
    y,
    r * theta.sin(),
  )
}
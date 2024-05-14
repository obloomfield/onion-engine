use ultraviolet as uv;

pub struct Camera {
    pub eye: uv::Vec3,
    pub target: uv::Vec3,
    pub up: uv::Vec3,
    pub fov: f32,
    pub aspect: f32,
    pub near: f32,
    pub far: f32,
}

impl Camera {
    pub fn new(
        eye: uv::Vec3,
        target: uv::Vec3,
        up: uv::Vec3,
        fov: f32,
        aspect: f32,
        near: f32,
        far: f32,
    ) -> Self {
        Self {
            eye,
            target,
            up,
            fov,
            aspect,
            near,
            far,
        }
    }

    pub fn build_view_projection_matrix(&self) -> uv::Mat4 {
        let view = uv::Mat4::look_at(self.eye, self.target, self.up);
        let proj = uv::projection::rh_yup::perspective_wgpu_dx(
            self.fov.to_radians(),
            self.aspect,
            self.near,
            self.far,
        );
        proj * view
    }
}

// We need this for Rust to store our data correctly for the shaders
#[repr(C)]
// This is so we can store this in a buffer
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
    // We can't use cgmath with bytemuck directly, so we'll have
    // to convert the Matrix4 into a 4x4 f32 array
    view_proj: [[f32; 4]; 4],
}

impl CameraUniform {
    pub fn new() -> Self {
        Self {
            view_proj: uv::Mat4::identity().into(),
        }
    }

    pub fn update_view_proj(&mut self, camera: &Camera) {
        self.view_proj = camera.build_view_projection_matrix().into();
    }
}

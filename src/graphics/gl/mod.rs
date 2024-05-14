#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub position: [f32; 3],
    pub tex_coords: [f32; 2],
}

pub struct BufferContents<'a> {
    pub vertices: &'a [Vertex],
    pub indices: &'a [u16],
    pub num_indices: u32,
}

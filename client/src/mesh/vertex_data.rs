#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct VertexData {
    pub position: [f32; 3],
    pub uv: [f32; 2],
    pub color: [u8; 4],
}

pub struct MeshData {
    pub vertices: Vec<VertexData>,
    pub indices: Option<Vec<u32>>,
}

use glam::Mat4;

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct InstanceData {
    pub model: [[f32; 4]; 4],
    pub color: [u8; 4],
    pub _pad: [u8; 12],
}

impl InstanceData {
    pub fn new(model: Mat4, color: [u8; 4]) -> Self {
        Self {
            model: model.to_cols_array_2d(),
            color,
            _pad: [0; 12],
        }
    }
}

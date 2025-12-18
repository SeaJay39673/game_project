use crate::engine::{GraphicsData, PlayerTexture};



pub struct Player {
    pub graphics_data: GraphicsData,
    pub position: (f32, f32),
    pub tex_type: PlayerTexture,
}
use serde::Deserialize;

use crate::engine::PlayerTexture;


#[derive(Deserialize)]
pub struct PlayerJson {
    pub name: PlayerTexture,
    pub index: [u32; 2]
}

#[derive(Deserialize)]
pub enum AssetJson {
    Player(Vec<PlayerJson>)
}

#[derive(Deserialize)]
pub struct AssetFileJson {
    pub path: String,
    pub x_count: u32,
    pub y_count: u32,
    pub assets: Vec<AssetJson>
}

#[derive(Deserialize)]
pub struct Json {
    pub asset_files: Vec<AssetFileJson>
}
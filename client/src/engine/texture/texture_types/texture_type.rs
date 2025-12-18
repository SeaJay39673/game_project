use crate::engine::{PlayerTexture};

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub enum TextureType {
    Player(PlayerTexture),
    Color([u8; 4]),
}
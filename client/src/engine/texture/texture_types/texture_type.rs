use crate::engine::Player;

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub enum TextureType {
    Player(Player),
    Color([u8; 4]),
}
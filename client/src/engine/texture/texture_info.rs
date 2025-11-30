
#[derive(Hash, PartialEq, Eq)]
pub struct TextureInfo {
    pub path: String,
    pub x_count: u32,
    pub y_count: u32,
}

impl TextureInfo {
    pub fn new(path: String, x_count: u32, y_count: u32) ->  Self {
        Self { path, x_count, y_count }
    }
}
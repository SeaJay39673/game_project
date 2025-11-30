use std::{
    collections::HashMap,
    sync::{Arc, LazyLock, RwLock, Weak},
};

use anyhow::anyhow;
use wgpu::{Device, Queue};

use crate::engine::{AssetInfo, AssetInfoType, ColorAssetInfo, Texture, TextureInfo, TextureType};

pub struct TextureAssetData {
    pub texture: Arc<Texture>,
    pub uv: [[f32; 2]; 4],
}

pub struct TextureManager {
    texture_atlas: HashMap<String, Weak<Texture>>,
    asset_info_atlas: HashMap<TextureType, AssetInfoType>,
    default_loaded: bool,
}

static TEXTURE_MANAGER: LazyLock<RwLock<TextureManager>> =
    LazyLock::new(|| RwLock::new(TextureManager::new()));

impl TextureManager {
    fn new() -> Self {
        Self {
            texture_atlas: HashMap::new(),
            asset_info_atlas: HashMap::new(),
            default_loaded: false,
        }
    }

    pub fn get_default_loaded() -> anyhow::Result<bool> {
        Ok(TEXTURE_MANAGER
            .read()
            .map_err(|e| anyhow!("Could not get TEXTURE_MANAGER for reading: {e}"))?
            .default_loaded
            .clone())
    }

    pub fn set_default_loaded(default_loaded: bool) -> anyhow::Result<()> {
        TEXTURE_MANAGER
            .write()
            .map_err(|e| anyhow!("Could not get TEXTURE_MANAGER for writing: {e}"))?
            .default_loaded = default_loaded;

        Ok(())
    }

    pub fn update_asset_info(
        texture_info: Arc<TextureInfo>,
        texture_type: TextureType,
        index: [u32; 2],
    ) -> anyhow::Result<()> {
        if let Some(asset_info) = TEXTURE_MANAGER
            .write()
            .map_err(|e| anyhow!("Could not get TEXTURE_MANAGER for writing: {e}"))?
            .asset_info_atlas
            .get_mut(&texture_type)
        {
            if let AssetInfoType::Asset(asset_info) = asset_info {
                if asset_info.texture_info != texture_info {
                    asset_info.update(texture_info, index);
                }
                return Ok(());
            }
        }

        let asset_info = AssetInfo::new(texture_info, index);
        TEXTURE_MANAGER
            .write()
            .map_err(|e| anyhow!("Could not get TEXTURE_MANAGER for writing: {e}"))?
            .asset_info_atlas
            .insert(texture_type, AssetInfoType::Asset(asset_info));

        Ok(())
    }

    pub fn update_color_asset_info(texture_type: TextureType) -> anyhow::Result<()> {
        let color = match texture_type {
            TextureType::Color(color) => color,
            _ => return Err(anyhow!("Non color texture type provided")),
        };

        if TEXTURE_MANAGER
            .read()
            .map_err(|e| anyhow!("Could not get TEXTURE_MANAGER for reading: {e}"))?
            .asset_info_atlas
            .get(&texture_type)
            .is_some()
        {
            return Ok(());
        }

        let color_asset_info = ColorAssetInfo::new(color);

        TEXTURE_MANAGER
            .write()
            .map_err(|e| anyhow!("Could not get TEXTURE_MANAGER for writing: {e}"))?
            .asset_info_atlas
            .insert(texture_type, AssetInfoType::Color(color_asset_info));

        Ok(())
    }

    pub fn get_texture_asset_data(
        device: &Device,
        queue: &Queue,
        texture_type: TextureType,
    ) -> anyhow::Result<TextureAssetData> {
        if let TextureType::Color(color) = texture_type {
            let mut manager = TEXTURE_MANAGER.write().map_err(|e| anyhow!("Could not get TEXTURE_MANAGER for writing: {e}"))?;
            if manager.asset_info_atlas.get(&texture_type).is_none() {
                let color_asset_info = ColorAssetInfo::new(color);
                manager.asset_info_atlas.insert(texture_type.clone(), AssetInfoType::Color(color_asset_info));
            }
        }
        
        
        let (path, uv) = {
            let manager = TEXTURE_MANAGER
                .read()
                .map_err(|e| anyhow!("Could not get TEXTURE_MANAGER for reading: {e}"))?;

            let asset_info = manager
                .asset_info_atlas
                .get(&texture_type)
                .ok_or(anyhow!("No asset_info for type {:?}", texture_type))?;

            let (path, uv) = match asset_info {
                AssetInfoType::Color(color_asset_info) => {
                    (format!("{:?}", color_asset_info.color), color_asset_info.uv)
                }
                AssetInfoType::Asset(asset_info) => {
                    (asset_info.texture_info.path.clone(), asset_info.uv)
                }
            };
            (path, uv)
        };

        let mut manager = TEXTURE_MANAGER
            .write()
            .map_err(|e| anyhow!("Could not get TEXTURE_MANAGER for writing: {e}"))?;

        if let Some(weak_tex) = manager.texture_atlas.get(&path) {
            if let Some(texture) = weak_tex.upgrade() {
                return Ok(TextureAssetData { texture, uv });
            } else {
                manager.texture_atlas.remove(&path);
            }
        }

        use TextureType::*;
        let texture: Arc<Texture> = match &texture_type {
            Color(color) => Arc::new(Texture::from_color(device, queue, color)),
            _ => Arc::new(Texture::from_file(device, queue, &path)?),
        };

        manager
            .texture_atlas
            .insert(path.clone(), Arc::downgrade(&texture));

        Ok(TextureAssetData { texture, uv })
    }
}

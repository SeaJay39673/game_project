use std::{fs, path::Path, sync::Arc};

use anyhow::anyhow;

use crate::{asset_ingestion::{AssetJson, Json, PlayerJson}, engine::{TextureType, TextureInfo, TextureManager}};

const DEFAULT_ASSETS: &str = "src/assets/assets.json";

fn load_players(tex_info: Arc<TextureInfo>, players: Vec<PlayerJson>) -> anyhow::Result<()>{
    for player in players {
        TextureManager::update_asset_info(Arc::clone(&tex_info), TextureType::Player(player.name), player.index)?;
    }
    Ok(())
}

pub fn load_assets(path: &str) -> anyhow::Result<()> {
    if ! TextureManager::get_default_loaded()? && path != DEFAULT_ASSETS {
        load_assets(DEFAULT_ASSETS)?;
    }
    
    let text = fs::read_to_string(path)?;
    let parsed: Json = serde_json::from_str(&text)?;

    let root = Path::new(path).parent().ok_or(anyhow!(
        "Could not get path provided for loading texture assets"
    ))?;

    use AssetJson::*;
    for asset_file in parsed.asset_files {
        let tex_info = Arc::new(TextureInfo::new(root.join(asset_file.path).to_string_lossy().to_string(), asset_file.x_count, asset_file.y_count));
        for asset in asset_file.assets {
            match asset {
                Player(players) => load_players(Arc::clone(&tex_info), players)?,         
            }
        }
    }

    if path == DEFAULT_ASSETS {
        TextureManager::set_default_loaded(true)?;
    }

    Ok(())
}

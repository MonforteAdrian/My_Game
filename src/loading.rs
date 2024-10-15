use crate::prelude::*;
use bevy::prelude::*;
use bevy::utils::HashMap;
use bevy_asset_loader::prelude::*;
use std::collections::BTreeMap;
use std::path::Path;

pub struct LoadingPlugin;

// TODO update the whole load assets
impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(GameState::Loading).continue_to_state(GameState::Splash),
        )
        .add_collection_to_loading_state::<_, FontAssets>(GameState::Loading)
        .add_collection_to_loading_state::<_, TextureAssets>(GameState::Loading)
        .init_resource_after_loading_state::<_, BlocksTextureAssets>(GameState::Loading)
        .init_resource_after_loading_state::<_, IconsTextureAssets>(GameState::Loading);
    }
}

#[derive(AssetCollection, Resource)]
pub struct FontAssets {
    #[asset(path = "fonts/FiraSans-Bold.ttf")]
    pub fira_sans: Handle<Font>,
}

#[derive(AssetCollection, Resource)]
pub struct TextureAssets {
    #[asset(path = "branding/icon.png")]
    pub bevy: Handle<Image>,

    #[asset(path = "icons", collection(typed, mapped))]
    pub icons: HashMap<String, Handle<Image>>,

    #[asset(path = "sprites/blocks", collection(typed, mapped))]
    pub blocks: HashMap<String, Handle<Image>>,
}

#[derive(Resource)]
pub struct IconsTextureAssets {
    pub icons_textures: BTreeMap<String, Handle<Image>>,
}

// Extract the file name and convert to ordered map
impl FromWorld for IconsTextureAssets {
    fn from_world(world: &mut World) -> Self {
        let image_assets = world.resource::<TextureAssets>();
        let ordered_images = image_assets
            .icons
            .clone()
            .into_iter()
            .map(|(path, value)| (extract_file_name(&path).to_string(), value))
            .collect();

        Self {
            icons_textures: ordered_images,
        }
    }
}

#[derive(Resource)]
pub struct BlocksTextureAssets {
    pub blocks_textures: BTreeMap<String, Handle<Image>>,
}

// Extract the file name and convert to ordered map
impl FromWorld for BlocksTextureAssets {
    fn from_world(world: &mut World) -> Self {
        let image_assets = world.resource::<TextureAssets>();
        let ordered_images = image_assets
            .blocks
            .clone()
            .into_iter()
            .map(|(path, value)| (extract_file_name(&path).to_string(), value))
            .collect();

        Self {
            blocks_textures: ordered_images,
        }
    }
}

// Extract the file name from the path without the extension
fn extract_file_name(path: &str) -> &str {
    Path::new(path)
        .file_name()
        .and_then(|f| f.to_str())
        .and_then(|f| f.split('.').next())
        .unwrap_or(path)
}

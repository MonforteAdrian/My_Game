use crate::prelude::*;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(AppState::Loading).continue_to_state(AppState::Splash),
        )
        .add_collection_to_loading_state::<_, TextureAssets>(AppState::Loading)
        .add_collection_to_loading_state::<_, FontAssets>(AppState::Loading);
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

    #[asset(path = "icons/arrow_right.png")]
    pub arrow_right: Handle<Image>,
    #[asset(path = "icons/exit_right.png")]
    pub exit_right: Handle<Image>,

    #[asset(path = "sprites/blocks/grass_block.png")]
    pub grass_block: Handle<Image>,
    #[asset(path = "sprites/blocks/grass_floor.png")]
    pub grass_floor: Handle<Image>,
    #[asset(path = "sprites/blocks/grass_half_block.png")]
    pub grass_half_block: Handle<Image>,
    #[asset(path = "sprites/blocks/grass_quarter_block.png")]
    pub grass_quarter_block: Handle<Image>,
    #[asset(path = "sprites/blocks/grass_ramp_top_left.png")]
    pub grass_ramp_top_left: Handle<Image>,
    #[asset(path = "sprites/blocks/grass_ramp_top_right.png")]
    pub grass_ramp_top_right: Handle<Image>,
    #[asset(path = "sprites/blocks/grass_stair_top_left.png")]
    pub grass_stair_top_left: Handle<Image>,
    #[asset(path = "sprites/blocks/grass_stair_top_right.png")]
    pub grass_stair_top_right: Handle<Image>,

    #[asset(path = "sprites/blocks/sand_block.png")]
    pub sand_block: Handle<Image>,
    #[asset(path = "sprites/blocks/sand_floor.png")]
    pub sand_floor: Handle<Image>,
    #[asset(path = "sprites/blocks/sand_half_block.png")]
    pub sand_half_block: Handle<Image>,
    #[asset(path = "sprites/blocks/sand_quarter_block.png")]
    pub sand_quarter_block: Handle<Image>,
    #[asset(path = "sprites/blocks/sand_ramp_top_left.png")]
    pub sand_ramp_top_left: Handle<Image>,
    #[asset(path = "sprites/blocks/sand_ramp_top_right.png")]
    pub sand_ramp_top_right: Handle<Image>,
    #[asset(path = "sprites/blocks/sand_stair_top_left.png")]
    pub sand_stair_top_left: Handle<Image>,
    #[asset(path = "sprites/blocks/sand_stair_top_right.png")]
    pub sand_stair_top_right: Handle<Image>,

    #[asset(path = "sprites/blocks/stone_block.png")]
    pub stone_block: Handle<Image>,
    #[asset(path = "sprites/blocks/stone_floor.png")]
    pub stone_floor: Handle<Image>,
    #[asset(path = "sprites/blocks/stone_half_block.png")]
    pub stone_half_block: Handle<Image>,
    #[asset(path = "sprites/blocks/stone_quarter_block.png")]
    pub stone_quarter_block: Handle<Image>,
    #[asset(path = "sprites/blocks/stone_ramp_top_left.png")]
    pub stone_ramp_top_left: Handle<Image>,
    #[asset(path = "sprites/blocks/stone_ramp_top_right.png")]
    pub stone_ramp_top_right: Handle<Image>,
    #[asset(path = "sprites/blocks/stone_stair_top_left.png")]
    pub stone_stair_top_left: Handle<Image>,
    #[asset(path = "sprites/blocks/stone_stair_top_right.png")]
    pub stone_stair_top_right: Handle<Image>,
}

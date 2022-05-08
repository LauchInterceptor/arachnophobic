use bevy_asset_loader::AssetCollection;

use crate::prelude::*;

#[derive(AssetCollection)]
pub struct TextureAtlasAssets {
    #[asset(texture_atlas(tile_size_x = 64., tile_size_y = 64., columns = 11, rows = 1))]
    #[asset(path = "player_ship/roll/roll_sheet.png")]
    pub player_ship: Handle<TextureAtlas>,
}

#[derive(AssetCollection)]
pub struct SpriteAssets {
    #[asset(path = "projectile/projectile_01.png")]
    pub projectile: Handle<Image>,
    #[asset(path = "enemy/spider_medium.png")]
    pub spider_medium: Handle<Image>,
    #[asset(path = "enemy/spider_small.png")]
    pub spider_small: Handle<Image>,
    #[asset(path = "enemy/spider_tiny.png")]
    pub spider_tiny: Handle<Image>,
}

#[derive(AssetCollection)]
pub struct FontAssets {
    #[asset(path = "font/FiraCode-Regular.ttf")]
    pub title_font: Handle<Font>,
}

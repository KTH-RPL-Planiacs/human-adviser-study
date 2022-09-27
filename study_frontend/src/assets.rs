use bevy::prelude::*;
use bevy::reflect::TypeUuid;
use bevy_asset_loader::prelude::*;
use serde::Deserialize;

use crate::{study::components::TileType, AppState};

#[derive(AssetCollection)]
pub struct FontAssets {
    #[asset(path = "fonts/FiraSans-Bold.ttf")]
    pub default_font: Handle<Font>,
}

#[derive(AssetCollection)]
pub struct MapAssets {
    #[asset(path = "sprites/tile_default.png")]
    pub default: Handle<Image>,
    #[asset(path = "sprites/tile_floor.png")]
    pub floor: Handle<Image>,
    #[asset(path = "sprites/tile_buns.png")]
    pub buns: Handle<Image>,
    #[asset(path = "sprites/tile_lettuce.png")]
    pub lettuce: Handle<Image>,
    #[asset(path = "sprites/tile_patty.png")]
    pub patty: Handle<Image>,
    #[asset(path = "sprites/tile_sauce.png")]
    pub sauce: Handle<Image>,
    #[asset(path = "sprites/tile_tomato.png")]
    pub tomato: Handle<Image>,
}

#[derive(AssetCollection)]
pub struct CharacterAssets {
    #[asset(path = "sprites/person.png")]
    pub person: Handle<Image>,
    #[asset(path = "sprites/robot.png")]
    pub robot: Handle<Image>,
}

#[derive(Deserialize, TypeUuid, Debug)]
#[uuid = "413be529-bfeb-41b3-9db0-4b8b380a2c46"]
pub struct TileData {
    pub floor: Vec<[usize; 2]>,
    pub patty: Vec<[usize; 2]>,
    pub buns: Vec<[usize; 2]>,
    pub tomato: Vec<[usize; 2]>,
    pub sauce: Vec<[usize; 2]>,
    pub lettuce: Vec<[usize; 2]>,
}

impl TileData {
    pub fn tile_by_coord(&self, x: usize, y: usize) -> TileType {
        if self.floor.contains(&[x, y]) {
            return TileType::Floor;
        }
        if self.patty.contains(&[x, y]) {
            return TileType::Patty;
        }
        if self.buns.contains(&[x, y]) {
            return TileType::Buns;
        }
        if self.tomato.contains(&[x, y]) {
            return TileType::Tomato;
        }
        if self.sauce.contains(&[x, y]) {
            return TileType::Sauce;
        }
        if self.lettuce.contains(&[x, y]) {
            return TileType::Lettuce;
        }

        TileType::Default
    }
}

pub fn setup_json(mut commands: Commands, asset_server: Res<AssetServer>) {
    let handle: Handle<TileData> = asset_server.load("data/tiles.json.tiles");
    commands.insert_resource(handle);
}

pub fn load_json(
    mut commands: Commands,
    handle: Res<Handle<TileData>>,
    mut tile_asset: ResMut<Assets<TileData>>,
    mut state: ResMut<State<AppState>>,
) {
    if let Some(tile_data) = tile_asset.remove(handle.id) {
        info!("{:?}", tile_data);
        commands.insert_resource(tile_data);
        state
            .set(AppState::MenuStart)
            .expect("Could not change state.");
    }
}

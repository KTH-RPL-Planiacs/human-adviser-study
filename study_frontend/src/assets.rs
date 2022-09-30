use bevy::reflect::TypeUuid;
use bevy::{prelude::*, utils::HashMap};
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

#[derive(AssetCollection)]
pub struct BurgerUiAssets {
    #[asset(path = "sprites/burger_inactive.png")]
    pub inactive: Handle<Image>,
    #[asset(path = "sprites/burger_patty.png")]
    pub patty: Handle<Image>,
    #[asset(path = "sprites/burger_buns.png")]
    pub buns: Handle<Image>,
    #[asset(path = "sprites/burger_tomato.png")]
    pub tomato: Handle<Image>,
    #[asset(path = "sprites/burger_sauce.png")]
    pub sauce: Handle<Image>,
    #[asset(path = "sprites/burger_lettuce.png")]
    pub lettuce: Handle<Image>,
}

#[derive(Deserialize, TypeUuid, Debug)]
#[uuid = "67c1c60e-2072-469a-8129-a46c8d1f80f2"]
pub struct TileData {
    pub floor: Vec<[usize; 2]>,
    pub patty: Vec<[usize; 2]>,
    pub buns: Vec<[usize; 2]>,
    pub tomato: Vec<[usize; 2]>,
    pub sauce: Vec<[usize; 2]>,
    pub lettuce: Vec<[usize; 2]>,
}

#[derive(Deserialize, TypeUuid, Debug)]
#[uuid = "58aa3298-015d-421d-b7d6-fa62a441f7f5"]
pub struct Strategy {
    pub strat: HashMap<String, String>,
    pub safe_edges: Vec<String>,
    pub fair_edges: Vec<String>,
}

#[derive(Deserialize, TypeUuid, Debug)]
#[uuid = "16ec115f-0c6f-4513-a2b1-7b07fedb5314"]
pub struct GameData {
    pub directed: bool,
    pub multigraph: bool,
    pub graph: Graph,
    pub nodes: Vec<NodeData>,
}

pub type GraphState = (String, String, String, u8);
#[derive(Deserialize, Debug)]
pub struct Graph {
    pub acc: Vec<GraphState>,
    pub init: GraphState,
    pub ap_r: Vec<String>,
    pub ap_h: Vec<String>,
}

#[derive(Deserialize, Debug)]
pub struct NodeData {
    pub player: u8,
    pub ap: Option<String>,
    pub id: GraphState,
}

#[derive(Deserialize, Debug)]
pub struct LinkData {
    pub act: Option<String>,
    pub prob: Option<f32>,
    pub source: GraphState,
    pub target: GraphState,
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
    let tile_handle: Handle<TileData> = asset_server.load("data/tiles.json.tiles");
    commands.insert_resource(tile_handle);
    let strat_handle: Handle<Strategy> = asset_server.load("data/strat.json.strat");
    commands.insert_resource(strat_handle);
    let game_handle: Handle<GameData> = asset_server.load("data/game.json.game");
    commands.insert_resource(game_handle);
}

pub fn load_tile_data(
    mut commands: Commands,
    tile_handle: Res<Handle<TileData>>,
    mut tile_asset: ResMut<Assets<TileData>>,
) {
    if let Some(tile_data) = tile_asset.remove(tile_handle.id) {
        commands.insert_resource(tile_data);
    }
}

pub fn load_strat_data(
    mut commands: Commands,
    strat_handle: Res<Handle<Strategy>>,
    mut strat_asset: ResMut<Assets<Strategy>>,
) {
    if let Some(strat_data) = strat_asset.remove(strat_handle.id) {
        commands.insert_resource(strat_data);
    }
}

pub fn load_game_data(
    mut commands: Commands,
    game_handle: Res<Handle<GameData>>,
    mut game_asset: ResMut<Assets<GameData>>,
) {
    if let Some(game_data) = game_asset.remove(game_handle.id) {
        info!("{:?}", game_data);
        commands.insert_resource(game_data);
    }
}

pub fn finish_loading(
    mut state: ResMut<State<AppState>>,
    tile_data: Option<Res<TileData>>,
    strategy: Option<Res<Strategy>>,
    synth_game: Option<Res<GameData>>,
) {
    if tile_data.is_some() && strategy.is_some() && synth_game.is_some() {
        state
            .set(AppState::MenuStart)
            .expect("Could not change state.");
    }
}

mod assets;
mod menu;
mod study;

use assets::*;
use bevy::{prelude::*, render::texture::ImageSettings};
use bevy_asset_loader::prelude::*;
use bevy_common_assets::json::JsonAssetPlugin;
use menu::{
    end::send_study_data,
    start::{update_part_id, update_part_id_display, update_start_btn},
};
use study::systems::*;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    AssetLoading,
    JsonLoading,
    MenuStart,
    Study,
    End,
}

fn main() {
    App::new()
        .insert_resource(ImageSettings::default_nearest())
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(WindowDescriptor {
            //fit_canvas_to_parent: true,
            height: 720.0,
            width: 1280.0,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(JsonAssetPlugin::<TileData>::new(&["json.tiles"]))
        .add_plugin(JsonAssetPlugin::<Strategy>::new(&["json.strat"]))
        .add_plugin(JsonAssetPlugin::<SynthGame>::new(&["json.game"]))
        .add_loading_state(
            LoadingState::new(AppState::AssetLoading)
                .continue_to_state(AppState::JsonLoading)
                .with_collection::<FontAssets>()
                .with_collection::<MapAssets>()
                .with_collection::<MenuAssets>()
                .with_collection::<CharacterAssets>()
                .with_collection::<BurgerUiAssets>(),
        )
        .add_state(AppState::AssetLoading)
        // json loading
        .add_system_set(SystemSet::on_enter(AppState::JsonLoading).with_system(setup_json))
        .add_system_set(
            SystemSet::on_update(AppState::JsonLoading)
                .with_system(load_tile_data)
                .with_system(load_strat_data)
                .with_system(load_game_data)
                .with_system(finish_loading),
        )
        // start menu
        .add_system_set(SystemSet::on_enter(AppState::MenuStart).with_system(menu::start::setup_ui))
        .add_system_set(
            SystemSet::on_update(AppState::MenuStart)
                .with_system(update_part_id)
                .with_system(update_part_id_display)
                .with_system(update_start_btn)
                .with_system(menu::start::btn_visuals)
                .with_system(menu::start::btn_listeners),
        )
        .add_system_set(
            SystemSet::on_exit(AppState::MenuStart).with_system(menu::start::cleanup_ui),
        )
        // study
        .add_system_set(
            SystemSet::on_enter(AppState::Study)
                .with_system(setup_burger_ui)
                .with_system(setup_adviser_ui)
                .with_system(setup_study)
                .with_system(setup_tiles)
                .with_system(setup_actors),
        )
        .add_system_set(
            SystemSet::on_update(AppState::Study)
                .with_system(window_resize_listener)
                .with_system(scale_burger_ui)
                .with_system(scale_adviser_ui)
                .with_system(resize_tiles)
                .with_system(resize_actors)
                .with_system(draw_actor_to_pos)
                .with_system(check_for_move)
                .with_system(resolve_moves),
        )
        .add_system_set(SystemSet::on_exit(AppState::Study).with_system(cleanup_study))
        // end
        .add_system_set(
            SystemSet::on_enter(AppState::End)
                .with_system(menu::end::setup_ui)
                .with_system(send_study_data),
        )
        .run();
}

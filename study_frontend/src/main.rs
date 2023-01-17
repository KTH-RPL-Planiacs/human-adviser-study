mod assets;
mod menu;
mod study;

use assets::*;
use bevy::{prelude::*, render::texture::ImageSettings};
use bevy_asset_loader::prelude::*;
use bevy_common_assets::json::JsonAssetPlugin;
use study::{logic_systems::*, ui_systems::*};
use study_shared_types::AdviserMode;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    AssetLoading,
    JsonLoading,
    MenuStart,
    Study,
    End,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, SystemLabel)]
enum SystemSetLabels {
    StudyUi,
    StudyLogic,
}

fn main() {
    // choose adviser mode at random
    /*
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let adviser_mode = match rng.gen_range(0..=2) {
        0 => AdviserMode::LeastLimiting,
        1 => AdviserMode::NextMove,
        _ => AdviserMode::None,
    };
    */

    App::new()
        .insert_resource(ImageSettings::default_nearest())
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(WindowDescriptor {
            height: 768.0,
            width: 1366.0,
            ..default()
        })
        .insert_resource(AdviserMode::NextMove)
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
                .with_collection::<AdviserAssets>()
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
                .label(SystemSetLabels::StudyLogic)
                .with_system(tick_timers)
                .with_system(prepare_human_move)
                .with_system(prepare_robot_move)
                .with_system(
                    resolve_moves
                        .after(prepare_human_move)
                        .after(prepare_robot_move),
                )
                .with_system(update_animation_state.after(resolve_moves))
                .with_system(update_animation_state.after(resolve_moves)),
        )
        .add_system_set(
            SystemSet::on_update(AppState::Study)
                .label(SystemSetLabels::StudyUi)
                .with_system(update_fade_away_sprite)
                .with_system(window_resize_listener)
                .with_system(scale_burger_ui)
                .with_system(scale_burger_ingredients_ui)
                .with_system(scale_burger_text_ui)
                .with_system(update_human_burger_ui)
                .with_system(update_robot_burger_ui)
                .with_system(scale_adviser_ui)
                .with_system(scale_adviser_text_ui)
                .with_system(update_timer_text)
                .with_system(update_burger_text)
                .with_system(resize_tiles)
                .with_system(resize_delivery_indicator)
                .with_system(toggle_delivery_indicator)
                .with_system(resize_actors)
                .with_system(resize_speech_bubble)
                .with_system(toggle_speech_bubble)
                .with_system(update_adviser_ui)
                .with_system(draw_actor_to_pos)
                .after(SystemSetLabels::StudyLogic),
        )
        .add_system_set(SystemSet::on_exit(AppState::Study).with_system(cleanup_study))
        // end
        .add_system_set(
            SystemSet::on_enter(AppState::End)
                .with_system(menu::end::setup_ui)
                .with_system(menu::end::send_study_data),
        )
        .run();
}

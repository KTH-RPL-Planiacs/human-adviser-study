mod menu;
mod study;

use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use menu::{
    end::send_study_data,
    start::{update_part_id, update_part_id_display, update_start_btn},
};
use study::systems::{cleanup_study, setup_study, update_study};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    AssetLoading,
    MenuStart,
    Study,
    End,
}

#[derive(AssetCollection)]
pub struct FontAssets {
    #[asset(path = "fonts/FiraSans-Bold.ttf")]
    pub default_font: Handle<Font>,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_loading_state(
            LoadingState::new(AppState::AssetLoading)
                .continue_to_state(AppState::MenuStart)
                .with_collection::<FontAssets>(),
        )
        .add_state(AppState::AssetLoading)
        // main menu
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
        .add_system_set(SystemSet::on_enter(AppState::Study).with_system(setup_study))
        .add_system_set(SystemSet::on_update(AppState::Study).with_system(update_study))
        .add_system_set(SystemSet::on_exit(AppState::Study).with_system(cleanup_study))
        // end
        .add_system_set(
            SystemSet::on_enter(AppState::End)
                .with_system(menu::end::setup_ui)
                .with_system(send_study_data),
        )
        .run();
}

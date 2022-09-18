mod menu;
mod study;

use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use menu::{
    end::send_study_data,
    main::{update_part_id, update_part_id_display, update_start_btn},
};
use study::systems::{cleanup_study, setup_study, update_study};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    AssetLoading,
    MenuMain,
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
                .continue_to_state(AppState::MenuMain)
                .with_collection::<FontAssets>(),
        )
        .add_state(AppState::AssetLoading)
        // main menu
        .add_system_set(SystemSet::on_enter(AppState::MenuMain).with_system(menu::main::setup_ui))
        .add_system_set(
            SystemSet::on_update(AppState::MenuMain)
                .with_system(update_part_id)
                .with_system(update_part_id_display)
                .with_system(update_start_btn)
                .with_system(menu::main::btn_visuals)
                .with_system(menu::main::btn_listeners),
        )
        .add_system_set(SystemSet::on_exit(AppState::MenuMain).with_system(menu::main::cleanup_ui))
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

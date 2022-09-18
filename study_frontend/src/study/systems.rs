use bevy::prelude::*;
use study_shared_types::GameResults;

use crate::{menu::main::ParticipantId, AppState};

pub fn setup_study() {}

pub fn update_study(mut state: ResMut<State<AppState>>) {
    state.set(AppState::End).expect("Could not change state.");
}

pub fn cleanup_study(mut commands: Commands, part_id: Res<ParticipantId>) {
    commands.insert_resource(GameResults {
        participant_id: part_id.0.parse::<i32>().unwrap(),
    })
}

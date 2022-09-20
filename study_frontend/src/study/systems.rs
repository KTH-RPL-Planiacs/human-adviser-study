use bevy::prelude::*;
use study_shared_types::GameResults;

use crate::{menu::start::ParticipantId, study::components::*, AppState, CharacterAssets};

pub fn setup_study(mut commands: Commands, player_sprites: Res<CharacterAssets>) {
    // 2d camera
    commands
        .spawn_bundle(Camera2dBundle::default())
        .insert(Study);

    // player and robot
    commands
        .spawn_bundle(SpriteBundle {
            texture: player_sprites.person.clone(),
            transform: Transform::from_xyz(100., 0., 0.),
            ..default()
        })
        .insert(Player)
        .insert(Study);

    commands
        .spawn_bundle(SpriteBundle {
            texture: player_sprites.robot.clone(),
            transform: Transform::from_xyz(150., 0., 0.),
            ..default()
        })
        .insert(Robot)
        .insert(Study);
}

pub fn update_study(mut state: ResMut<State<AppState>>) {
    //state.set(AppState::End).expect("Could not change state.");
}

pub fn cleanup_study(
    query: Query<Entity, With<Study>>,
    mut commands: Commands,
    part_id: Res<ParticipantId>,
) {
    commands.insert_resource(GameResults {
        participant_id: part_id
            .0
            .parse::<i32>()
            .expect("The participant ID should be a number!"),
    });

    for e in query.iter() {
        commands.entity(e).despawn_recursive();
    }
}

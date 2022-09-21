use bevy::{prelude::*, window::WindowResized};
use study_shared_types::GameResults;

use crate::{menu::start::ParticipantId, study::components::*, AppState, CharacterAssets};

use super::{NUM_TILES, PADDING};

pub fn setup_study(
    mut commands: Commands,
    player_sprites: Res<CharacterAssets>,
    mut windows: ResMut<Windows>,
) {
    // 2d camera
    commands
        .spawn_bundle(Camera2dBundle::default())
        .insert(Study);

    // parameters
    let window = windows.get_primary_mut().unwrap();
    let width = window.width();
    let height = window.height();
    let tile_size = (width.min(height) / NUM_TILES as f32) - (PADDING * 2.0);
    commands.insert_resource(TileSize(tile_size));

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

pub fn window_resize_listener(
    resize_event: Res<Events<WindowResized>>,
    mut tile_size: ResMut<TileSize>,
) {
    let mut reader = resize_event.get_reader();
    for e in reader.iter(&resize_event) {
        let new_tile_size = (e.width.min(e.height) / NUM_TILES as f32) - (PADDING * 2.0);
        tile_size.0 = new_tile_size;
    }
}

pub fn check_for_move(mut commands: Commands, keyboard_input: Res<Input<KeyCode>>) {
    let mut next_move: Option<NextMove> = None;

    if keyboard_input.just_pressed(KeyCode::Left) {
        next_move = Some(NextMove::Left);
    }

    if keyboard_input.just_pressed(KeyCode::Right) {
        next_move = Some(NextMove::Right)
    }

    if keyboard_input.just_pressed(KeyCode::Up) {
        next_move = Some(NextMove::Up)
    }

    if keyboard_input.just_pressed(KeyCode::Down) {
        next_move = Some(NextMove::Down)
    }

    if keyboard_input.just_pressed(KeyCode::Space) {
        next_move = Some(NextMove::Interact)
    }

    if let Some(m) = next_move {
        commands.insert_resource(m);
    }
}

pub fn resolve_move(
    mut commands: Commands,
    next_move: Option<Res<NextMove>>,
    mut state: ResMut<State<AppState>>,
) {
    if let Some(m) = next_move {
        info!("{:?}", m);

        match *m {
            NextMove::Up => (),
            NextMove::Down => (),
            NextMove::Left => (),
            NextMove::Right => (),
            NextMove::Interact => state.set(AppState::End).expect("Could not change state."),
        }

        commands.remove_resource::<NextMove>();
    }
}

pub fn update_study() {
    //;
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

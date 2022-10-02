use bevy::prelude::*;
use study_shared_types::GameResults;

use crate::{
    assets::{
        CharacterAssets, GraphState, MapAssets, Strategy, SynthGame, SynthGameState, TileData,
    },
    menu::start::ParticipantId,
    study::components::*,
};

use super::{ANIM_DURATION, NUM_TILES, PADDING};

/*
*   SETUP
*/

pub fn setup_study(mut commands: Commands, windows: Res<Windows>) {
    commands.insert_resource(StudyState::Idle);
    commands.insert_resource(AnimationTimer(Timer::new(ANIM_DURATION, false)));
    commands.insert_resource(BurgerStatus::default());

    // 2d camera
    commands
        .spawn_bundle(Camera2dBundle::default())
        .insert(Study);

    // tile size
    let window = windows.get_primary().unwrap();
    let width = window.width();
    let height = window.height();
    let size_min = width.min(height);
    let tile_size = (size_min - (PADDING * 2.0)) / NUM_TILES as f32;
    commands.insert_resource(WindowSize { width, height });
    commands.insert_resource(TileSize(tile_size));
}

pub fn setup_tiles(mut commands: Commands, tile_data: Res<TileData>, tile_sprites: Res<MapAssets>) {
    for x in 0..NUM_TILES {
        for y in 0..NUM_TILES {
            let tile_type = tile_data.tile_by_coord(x, y);
            let tile_texture = match tile_type {
                TileType::Default => tile_sprites.default.clone(),
                TileType::Floor => tile_sprites.floor.clone(),
                TileType::Buns => tile_sprites.buns.clone(),
                TileType::Patty => tile_sprites.patty.clone(),
                TileType::Lettuce => tile_sprites.lettuce.clone(),
                TileType::Tomato => tile_sprites.tomato.clone(),
                TileType::Sauce => tile_sprites.sauce.clone(),
            };

            commands
                .spawn_bundle(SpriteBundle {
                    texture: tile_texture,
                    ..default()
                })
                .insert(tile_type)
                .insert(Tile { x, y })
                .insert(Study);
        }
    }
}

const HUMAN_START: (usize, usize) = (2, 4);
const ROBOT_START: (usize, usize) = (2, 0);
pub fn setup_actors(mut commands: Commands, player_sprites: Res<CharacterAssets>) {
    // player
    commands
        .spawn_bundle(SpriteBundle {
            texture: player_sprites.person.clone(),
            transform: Transform::from_xyz(0., 0., 1.),
            ..default()
        })
        .insert(Position {
            x: HUMAN_START.0,
            y: HUMAN_START.1,
        })
        .insert(NextPosition {
            x: HUMAN_START.0,
            y: HUMAN_START.1,
        })
        .insert(Player)
        .insert(Study);

    // robot
    commands
        .spawn_bundle(SpriteBundle {
            texture: player_sprites.robot.clone(),
            transform: Transform::from_xyz(0., 0., 1.),
            ..default()
        })
        .insert(Position {
            x: ROBOT_START.0,
            y: ROBOT_START.1,
        })
        .insert(NextPosition {
            x: ROBOT_START.0,
            y: ROBOT_START.1,
        })
        .insert(Robot)
        .insert(Study);
}

/*
*   UPDATE
*/

pub fn prepare_robot_move(
    mut commands: Commands,
    mut synth_game_state: ResMut<SynthGameState>,
    strategy: Res<Strategy>,
    synth_game: Res<SynthGame>,
    robot_move: Option<Res<RobotNextMove>>,
) {
    if robot_move.is_none() {
        let robot_move = if let Some(next_move) = strategy.next_move(&synth_game_state.0) {
            next_move
        } else {
            let valid_moves = synth_game.valid_moves(&synth_game_state.0);
            valid_moves[0]
        };

        if synth_game.is_accepting(&synth_game_state.0) {
            // TODO: OVERRIDE WITH DELIVERY
            info!("ACCEPT!");
        }

        // get next state from game
        let prob_state: GraphState = synth_game.next_state(&synth_game_state.0, robot_move);
        let human_state: GraphState = synth_game.skip_prob_state(&prob_state);
        synth_game_state.0 = human_state;

        commands.insert_resource(RobotNextMove(robot_move))
    }
}

pub fn prepare_human_move(mut commands: Commands, keyboard_input: Res<Input<KeyCode>>) {
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
        commands.insert_resource(HumanNextMove(m));
    }
}

pub fn resolve_moves(
    mut commands: Commands,
    mut player: Query<(&Position, &mut NextPosition), (With<Player>, Without<Robot>)>,
    mut robot: Query<(&Position, &mut NextPosition), (With<Robot>, Without<Player>)>,
    mut study_state: ResMut<StudyState>,
    mut anim_timer: ResMut<AnimationTimer>,
    mut synth_game_state: ResMut<SynthGameState>,
    synth_game: Res<SynthGame>,
    next_move_r: Option<ResMut<RobotNextMove>>,
    next_move_h: Option<ResMut<HumanNextMove>>,
) {
    // we only apply the next move if the simulation is ready
    if *study_state != StudyState::Idle {
        return;
    }

    if let Some(mut human_move) = next_move_h {
        if let Some(robot_move) = next_move_r {
            commands.remove_resource::<HumanNextMove>();
            commands.remove_resource::<RobotNextMove>();

            // reset animation timer
            *study_state = StudyState::Animation;
            anim_timer.0.reset();

            // make sure the human move is valid, if not, just pick the first valid one
            let valid_moves = synth_game.valid_moves(&synth_game_state.0);
            if !valid_moves.contains(&human_move.0) {
                human_move.0 = valid_moves[0];
            }

            // get next state from game
            let prob_state: GraphState = synth_game.next_state(&synth_game_state.0, human_move.0);
            synth_game_state.0 = synth_game.skip_prob_state(&prob_state);

            // update grid positions for position interpolation
            let (cur_pos_r, mut next_pos_r) = robot
                .get_single_mut()
                .expect("There should only be one robot.");

            let (cur_pos_h, mut next_pos_h) = player
                .get_single_mut()
                .expect("There should only be one human.");

            *next_pos_r = next_pos_from_move(cur_pos_r, robot_move.0);
            *next_pos_h = next_pos_from_move(cur_pos_h, human_move.0);
        }
    }
}

fn next_pos_from_move(cur_pos: &Position, next_move: NextMove) -> NextPosition {
    match next_move {
        NextMove::Idle => {
            return NextPosition {
                x: cur_pos.x,
                y: cur_pos.y,
            };
        }
        NextMove::Up => {
            return NextPosition {
                x: cur_pos.x,
                y: cur_pos.y + 1,
            };
        }
        NextMove::Down => {
            return NextPosition {
                x: cur_pos.x,
                y: cur_pos.y - 1,
            };
        }
        NextMove::Left => {
            return NextPosition {
                x: cur_pos.x - 1,
                y: cur_pos.y,
            };
        }
        NextMove::Right => {
            return NextPosition {
                x: cur_pos.x + 1,
                y: cur_pos.y,
            };
        }
        NextMove::Interact => {
            return NextPosition {
                x: cur_pos.x,
                y: cur_pos.y,
            };
        }
    }
}

/*
*   CLEANUP
*/

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

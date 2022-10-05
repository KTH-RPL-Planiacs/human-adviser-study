use bevy::prelude::*;
use study_shared_types::GameResults;

use crate::{
    assets::{
        CharacterAssets, GraphState, MapAssets, Strategy, SynthGame, SynthGameState, TileData,
    },
    menu::start::ParticipantId,
    study::components::*,
};

use super::*;

/*
*   SETUP
*/

pub fn setup_study(mut commands: Commands, windows: Res<Windows>) {
    commands.insert_resource(StudyState::Idle);
    commands.insert_resource(AnimationTimer(Timer::new(ANIM_DURATION, false)));
    commands.insert_resource(BurgerProgress::default());

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
        .insert(Interact::No)
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
        .insert(Interact::No)
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
    mut player: Query<
        (&Position, &mut Interact, &mut NextPosition),
        (With<Player>, Without<Robot>),
    >,
    mut robot: Query<(&Position, &mut Interact, &mut NextPosition), (With<Robot>, Without<Player>)>,
    mut study_state: ResMut<StudyState>,
    mut anim_timer: ResMut<AnimationTimer>,
    mut synth_game_state: ResMut<SynthGameState>,
    mut burger_progress: ResMut<BurgerProgress>,
    synth_game: Res<SynthGame>,
    next_move_r: Option<ResMut<RobotNextMove>>,
    next_move_h: Option<ResMut<HumanNextMove>>,
) {
    // we only apply the next move if the simulation is ready
    if *study_state != StudyState::Idle {
        return;
    }

    let robot_move = if let Some(m) = next_move_r {
        m.0
    } else {
        return;
    };

    let valid_moves = synth_game.valid_moves(&synth_game_state.0);
    let human_move = if let Some(m) = next_move_h {
        // make sure the human move is valid, if not, just pick the first valid one
        if valid_moves.contains(&m.0) {
            m.0
        } else {
            valid_moves[0]
        }
    } else if valid_moves.len() == 1 {
        // if the only option is to interact, we queue the move
        valid_moves[0]
    } else {
        return;
    };

    commands.remove_resource::<HumanNextMove>();
    commands.remove_resource::<RobotNextMove>();

    // reset animation timer
    *study_state = StudyState::Animation;
    anim_timer.0.reset();

    // fetch current and next positions
    let (cur_pos_r, mut interact_r, mut next_pos_r) = robot
        .get_single_mut()
        .expect("There should only be one robot.");
    let (cur_pos_h, mut interact_h, mut next_pos_h) = player
        .get_single_mut()
        .expect("There should only be one human.");

    // interaction - human
    if human_move == NextMove::Interact {
        let interact_pos_h = interacting_pos(cur_pos_h);
        if valid_moves.len() == 1 {
            *interact_h = Interact::Out(interact_pos_h);
            update_burger_status(&mut burger_progress, cur_pos_h);
        } else {
            *interact_h = Interact::In(interact_pos_h);
        }
    } else {
        *interact_h = Interact::No;
    }

    // interaction - robot
    if robot_move == NextMove::Interact {
        let interact_pos_r = interacting_pos(cur_pos_r);
        if valid_moves.len() == 1 {
            *interact_r = Interact::Out(interact_pos_r);
        } else {
            *interact_r = Interact::In(interact_pos_r);
        }
    } else {
        *interact_r = Interact::No;
    }

    // get next state from game
    let prob_state: GraphState = synth_game.next_state(&synth_game_state.0, human_move);
    synth_game_state.0 = synth_game.skip_prob_state(&prob_state);

    // update grid positions for position interpolation
    *next_pos_r = next_pos_from_move(cur_pos_r, robot_move);
    *next_pos_h = next_pos_from_move(cur_pos_h, human_move);
}

fn interacting_pos(cur_pos: &Position) -> Position {
    if cur_pos.x == DELIVERY_POS_H.0 && cur_pos.y == DELIVERY_POS_H.1 {
        return Position {
            x: cur_pos.x - 1,
            y: cur_pos.y,
        };
    }

    // TODO: actual position
    return Position {
        x: cur_pos.x,
        y: cur_pos.y,
    };
}

fn update_burger_status(burger_progress: &mut BurgerProgress, cur_pos: &Position) {
    if cur_pos.x == DELIVERY_POS_H.0 && cur_pos.y == DELIVERY_POS_H.1 {
        burger_progress.make_burger();
    }

    if cur_pos.x == PATTY_POS_H.0 && cur_pos.y == PATTY_POS_H.1 {
        burger_progress.patty = true;
    }

    if cur_pos.x == BUNS_POS_H.0 && cur_pos.y == BUNS_POS_H.1 {
        burger_progress.buns = true;
    }

    if cur_pos.x == LETTUCE_POS_H.0 && cur_pos.y == LETTUCE_POS_H.1 {
        burger_progress.lettuce = true;
    }

    if cur_pos.x == TOMATO_POS_H.0 && cur_pos.y == TOMATO_POS_H.1 {
        burger_progress.tomato = true;
    }

    if cur_pos.x == SAUCE_POS_H.0 && cur_pos.y == SAUCE_POS_H.1 {
        burger_progress.sauce = true;
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

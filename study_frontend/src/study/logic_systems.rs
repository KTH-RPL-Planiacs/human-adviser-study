use bevy::prelude::*;
use study_shared_types::GameResults;

use crate::{
    assets::{
        CharacterAssets, GraphState, MapAssets, Strategy, SynthGame, SynthGameState, TileData,
    },
    menu::start::ParticipantId,
    study::components::*,
    AppState,
};

use super::*;

/*
*   SETUP
*/

pub fn setup_study(mut commands: Commands, windows: Res<Windows>, part_id: Res<ParticipantId>) {
    commands.insert_resource(StudyState::Idle);
    commands.insert_resource(AnimationTimer(Timer::new(ANIM_DURATION, false)));
    commands.insert_resource(GameTimer(Timer::new(GAME_DURATION, false)));
    commands.insert_resource(BurgerProgress::default());
    commands.insert_resource(ActiveAdvisers::default());
    commands.insert_resource(GameResults {
        participant_id: part_id
            .0
            .parse::<i32>()
            .expect("The participant ID should be a number!"),
        human_burgers: 0,
        robot_burgers: 0,
    });

    // 2d camera
    commands
        .spawn_bundle(Camera2dBundle::default())
        .insert(Study);

    // tile size
    let window = windows.get_primary().unwrap();
    let width = window.width();
    let height = window.height();
    let size_min = width.min(height);
    let tile_size = (size_min - (TILE_PADDING * 2.0)) / NUM_TILES as f32;
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

pub fn setup_actors(mut commands: Commands, player_sprites: Res<CharacterAssets>) {
    // player
    commands
        .spawn_bundle(SpriteBundle {
            texture: player_sprites.person.clone(),
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

    // fade away screen sprite
    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform::from_xyz(0., 0., MENU_Z - 1.),
            sprite: Sprite {
                color: Color::BLACK,
                ..default()
            },
            ..default()
        })
        .insert(FadeAwayScreen)
        .insert(Study);
}

/*
*   UPDATE
*/

pub fn update_animation_state(
    mut commands: Commands,
    anim_timer: Res<AnimationTimer>,
    mut study_state: ResMut<StudyState>,
    is_violated: Option<Res<SafetyViolated>>,
    mut player: Query<
        (&mut Position, &mut Interact, &mut NextPosition),
        (With<Player>, Without<Robot>),
    >,
    mut robot: Query<
        (&mut Position, &mut Interact, &mut NextPosition),
        (With<Robot>, Without<Player>),
    >,
    mut burger_progress: ResMut<BurgerProgress>,
    mut synth_game_state: ResMut<SynthGameState>,
    synth_game: Res<SynthGame>,
) {
    // if animation is over, we reset animation state
    if anim_timer.0.finished() {
        *study_state = StudyState::Idle;

        // if we had a safety violation, reset simulation
        if is_violated.is_some() {
            if let Ok((mut pos, mut interact, mut next_pos)) = player.get_single_mut() {
                pos.x = HUMAN_START.0;
                pos.y = HUMAN_START.1;
                next_pos.x = HUMAN_START.0;
                next_pos.y = HUMAN_START.1;
                *interact = Interact::No;
            }
            if let Ok((mut pos, mut interact, mut next_pos)) = robot.get_single_mut() {
                pos.x = ROBOT_START.0;
                pos.y = ROBOT_START.1;
                next_pos.x = ROBOT_START.0;
                next_pos.y = ROBOT_START.1;
                *interact = Interact::No;
            }
            burger_progress.reset();
            synth_game_state.0 = synth_game.graph.init.clone();
            commands.remove_resource::<SafetyViolated>();
            commands.remove_resource::<RobotNextMove>();
            commands.remove_resource::<HumanNextMove>();
        }
    }
}

pub fn tick_timers(
    mut anim: ResMut<AnimationTimer>,
    mut game: ResMut<GameTimer>,
    time: Res<Time>,
    mut state: ResMut<State<AppState>>,
) {
    let delta = time.delta();
    anim.0.tick(delta);
    game.0.tick(delta);

    if game.0.finished() {
        state.set(AppState::End).expect("Could not change state.");
    }
}

pub fn prepare_robot_move(
    mut commands: Commands,
    mut synth_game_state: ResMut<SynthGameState>,
    mut active_advisers: ResMut<ActiveAdvisers>,
    strategy: Res<Strategy>,
    synth_game: Res<SynthGame>,
    robot_next_move: Option<Res<RobotNextMove>>,
    mut game_results: ResMut<GameResults>,
    adviser_icons: Query<Entity, With<AdviserIcon>>,
) {
    if robot_next_move.is_none() {
        let mut robot_move = if let Some(next_move) = strategy.next_move(&synth_game_state.0) {
            next_move
        } else {
            let valid_moves = synth_game.valid_robot_moves(&synth_game_state.0);
            valid_moves[0]
        };

        // if done with LTL task, go to delivery and interact, then reset
        if synth_game.is_accepting(&synth_game_state.0) {
            let robot_state_str = synth_game_state.0 .0.as_str();
            robot_move = delivery_move(robot_state_str);
            if robot_state_str == "20i" {
                // resets game to almost initial state
                synth_game_state.0 = synth_game.graph.init.clone();
                synth_game_state.0 .0 = "20i".to_string();
                game_results.robot_burgers += 1;
            }
        }

        // get next state from game
        let human_state: GraphState = synth_game.apply_robot_move(&synth_game_state.0, robot_move);
        synth_game_state.0 = human_state;

        commands.insert_resource(RobotNextMove(robot_move));

        // update avisers
        active_advisers.clear_all();
        for adv_icon in adviser_icons.iter() {
            commands.entity(adv_icon).despawn_recursive();
        }
        for safe_adv in &strategy.safety_adv {
            let state_from: &GraphState = &safe_adv.0;
            if synth_game_state.0 == *state_from {
                let guards = safe_adv.1.clone();
                assert!(guards.len() == 1);
                active_advisers.safety.extend(guards);
            }
        }
        for fair_adv in &strategy.fairness_adv {
            let state_from: &GraphState = &fair_adv.0;
            if synth_game_state.0 == *state_from {
                let guards = fair_adv.1.clone();
                assert!(guards.len() == 1);
                active_advisers.fairness.extend(guards);
            }
        }
    }
}

fn valid_human_moves(cur_pos: &Position, interact: &Interact) -> Vec<NextMove> {
    // if we just went into interaction, we can only finish it
    if let Interact::In(_) = interact {
        return vec![NextMove::Interact];
    }

    if cur_pos.is_equal(DELIVERY_POS_H) {
        return vec![NextMove::Idle, NextMove::Interact, NextMove::Down];
    }

    if cur_pos.is_equal(PATTY_POS_H) {
        return vec![NextMove::Idle, NextMove::Interact, NextMove::Right];
    }

    if cur_pos.is_equal(BUNS_POS_H) {
        return vec![
            NextMove::Idle,
            NextMove::Interact,
            NextMove::Left,
            NextMove::Right,
        ];
    }

    if cur_pos.is_equal(TOMATO_POS_H) {
        return vec![
            NextMove::Idle,
            NextMove::Interact,
            NextMove::Up,
            NextMove::Left,
            NextMove::Right,
        ];
    }

    if cur_pos.is_equal(SAUCE_POS_H) {
        return vec![
            NextMove::Idle,
            NextMove::Interact,
            NextMove::Left,
            NextMove::Right,
        ];
    }

    if cur_pos.is_equal(LETTUCE_POS_H) {
        return vec![NextMove::Idle, NextMove::Interact, NextMove::Left];
    }

    panic!("valid_human_moves(): Could not find valid moves!");
}

fn delivery_move(state: &str) -> NextMove {
    match state {
        "01" => NextMove::Right,
        "11" => NextMove::Right,
        "21" => NextMove::Down,
        "31" => NextMove::Left,
        "41" => NextMove::Left,
        "01i" => NextMove::Interact,
        "11i" => NextMove::Interact,
        "21i" => NextMove::Interact,
        "31i" => NextMove::Interact,
        "41i" => NextMove::Interact,
        "20" => NextMove::Interact,
        "20i" => NextMove::Interact,
        _ => panic!("delivery_move({:?}): No hardcoded move found!", state),
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
    mut game_results: ResMut<GameResults>,
    active_advisers: Res<ActiveAdvisers>,
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

    // fetch current and next positions
    let (cur_pos_r, mut interact_r, mut next_pos_r) = robot
        .get_single_mut()
        .expect("There should only be one robot.");
    let (cur_pos_h, mut interact_h, mut next_pos_h) = player
        .get_single_mut()
        .expect("There should only be one human.");

    let valid_moves = valid_human_moves(&cur_pos_h, interact_h.as_ref());
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
    anim_timer.0.reset();

    // interaction - human
    if human_move == NextMove::Interact {
        let interact_pos_h = interacting_pos(cur_pos_h);
        if valid_moves.len() == 1 {
            *interact_h = Interact::Out(interact_pos_h);
            if update_burger_status(&mut burger_progress, cur_pos_h) {
                game_results.human_burgers += 1;
            }
        } else {
            *interact_h = Interact::In(interact_pos_h);
        }
    } else {
        *interact_h = Interact::No;
    }

    // interaction - robot
    if robot_move == NextMove::Interact {
        let interact_pos_r = interacting_pos(cur_pos_r);
        if matches!(*interact_r, Interact::In(_)) {
            *interact_r = Interact::Out(interact_pos_r);
        } else {
            *interact_r = Interact::In(interact_pos_r);
        }
    } else {
        *interact_r = Interact::No;
    }

    // update grid positions for position interpolation
    *next_pos_r = next_pos_from_move(cur_pos_r, robot_move);
    *next_pos_h = next_pos_from_move(cur_pos_h, human_move);

    // update synthesis game state
    let obs = obs_from_pos(next_pos_h.as_pos(), &interact_h, &synth_game.graph.human_ap);
    let prob_state: GraphState = synth_game.apply_human_obs(&synth_game_state.0, &obs);
    synth_game_state.0 = synth_game.skip_prob_state(&prob_state);

    // check for safety assumption violation
    // then update study state accordingly
    if active_advisers.safety_violated(&obs) {
        commands.insert_resource(SafetyViolated);
        anim_timer.0.set_duration(FADE_DURATION);
        *study_state = StudyState::FadeAway;
    } else {
        anim_timer.0.set_duration(ANIM_DURATION);
        *study_state = StudyState::Animation;
    }
}

fn obs_from_pos(pos: Position, interact: &Interact, guard_ap: &[String]) -> String {
    let mut obs = String::with_capacity(guard_ap.len());
    for ap in guard_ap {
        match ap.as_str() {
            "patty_h" => obs.push(interact_char(pos, interact, PATTY_POS_H)),
            "buns_h" => obs.push(interact_char(pos, interact, BUNS_POS_H)),
            "lettuce_h" => obs.push(interact_char(pos, interact, LETTUCE_POS_H)),
            "tomato_h" => obs.push(interact_char(pos, interact, TOMATO_POS_H)),
            "ketchup_h" => obs.push(interact_char(pos, interact, SAUCE_POS_H)),
            "delivery_h" => obs.push(interact_char(pos, interact, DELIVERY_POS_H)),
            _ => obs.push('0'),
        }
    }
    obs
}

fn interact_char(pos: Position, interact: &Interact, check: (usize, usize)) -> char {
    if let Interact::In(_) = interact {
        if pos.is_equal(check) {
            '1'
        } else {
            '0'
        }
    } else {
        '0'
    }
}

fn interacting_pos(cur_pos: &Position) -> Position {
    if cur_pos.is_equal(DELIVERY_POS_H) || cur_pos.is_equal(DELIVERY_POS_R) {
        return Position {
            x: cur_pos.x - 1,
            y: cur_pos.y,
        };
    }

    if cur_pos.is_equal(PATTY_POS_H)
        || cur_pos.is_equal(BUNS_POS_H)
        || cur_pos.is_equal(LETTUCE_POS_H)
        || cur_pos.is_equal(TOMATO_POS_H)
        || cur_pos.is_equal(SAUCE_POS_H)
    {
        return Position {
            x: cur_pos.x,
            y: cur_pos.y - 1,
        };
    }

    if cur_pos.is_equal(PATTY_POS_R)
        || cur_pos.is_equal(BUNS_POS_R)
        || cur_pos.is_equal(LETTUCE_POS_R)
        || cur_pos.is_equal(TOMATO_POS_R)
        || cur_pos.is_equal(SAUCE_POS_R)
    {
        return Position {
            x: cur_pos.x,
            y: cur_pos.y + 1,
        };
    }

    panic!("No interacting_pos found!");
}

// returns true if a burger was made
fn update_burger_status(burger_progress: &mut BurgerProgress, cur_pos: &Position) -> bool {
    if cur_pos.x == DELIVERY_POS_H.0 && cur_pos.y == DELIVERY_POS_H.1 {
        return burger_progress.make_burger();
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
    return false;
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

pub fn cleanup_study(query: Query<Entity, With<Study>>, mut commands: Commands) {
    for e in query.iter() {
        commands.entity(e).despawn_recursive();
    }
}

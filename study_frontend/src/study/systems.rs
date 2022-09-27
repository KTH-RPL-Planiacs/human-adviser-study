use bevy::{prelude::*, window::WindowResized};
use study_shared_types::GameResults;

use crate::{
    assets::{BurgerUiAssets, CharacterAssets, MapAssets, TileData},
    menu::start::ParticipantId,
    study::components::*,
    AppState,
};

use super::{ANIM_DURATION, BURGER_UI_WIDTH, MENU_Z, NUM_TILES, PADDING};

/*
*   SETUP
*/

pub fn setup_study(mut commands: Commands, windows: Res<Windows>) {
    commands.insert_resource(StudyState::Idle);
    commands.insert_resource(AnimationTimer(Timer::new(ANIM_DURATION, false)));

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

pub fn setup_actors(mut commands: Commands, player_sprites: Res<CharacterAssets>) {
    // player
    commands
        .spawn_bundle(SpriteBundle {
            texture: player_sprites.person.clone(),
            transform: Transform::from_xyz(0., 0., 1.),
            ..default()
        })
        .insert(Position { x: 2, y: 0 })
        .insert(NextPosition { x: 2, y: 0 })
        .insert(Player)
        .insert(Study);

    // robot
    commands
        .spawn_bundle(SpriteBundle {
            texture: player_sprites.robot.clone(),
            transform: Transform::from_xyz(0., 0., 1.),
            ..default()
        })
        .insert(Position { x: 2, y: 4 })
        .insert(NextPosition { x: 2, y: 4 })
        .insert(Robot)
        .insert(Study);
}

pub fn setup_burger_ui(mut commands: Commands, burger_sprites: Res<BurgerUiAssets>) {
    commands
        .spawn_bundle(SpriteBundle {
            texture: burger_sprites.inactive.clone(),
            ..default()
        })
        .insert(Study)
        .insert(BurgerUi)
        .add_children(|parent| {});
}

/*
*   UPDATE
*/

pub fn window_resize_listener(
    resize_event: Res<Events<WindowResized>>,
    mut tile_size: ResMut<TileSize>,
    mut window_size: ResMut<WindowSize>,
) {
    let mut reader = resize_event.get_reader();
    for e in reader.iter(&resize_event) {
        let size_min = e.width.min(e.height);
        let new_tile_size = (size_min - (PADDING * 2.0)) / NUM_TILES as f32;
        tile_size.0 = new_tile_size;
        window_size.width = e.width;
        window_size.height = e.height;
    }
}

pub fn scale_burger_ui(
    mut burger_ui: Query<(&mut Transform, &mut Sprite), With<BurgerUi>>,
    window_size: Res<WindowSize>,
) {
    if window_size.is_changed() {
        let (mut transf, mut sprite) = burger_ui.single_mut();
        let x_pos = window_size.width * -0.5 + BURGER_UI_WIDTH * 0.5;
        transf.translation = Vec3::new(x_pos, 0., MENU_Z);
        sprite.custom_size = Some(Vec2::new(BURGER_UI_WIDTH, window_size.height));
    }
}

pub fn resize_tiles(
    tile_size: Res<TileSize>,
    mut tiles: Query<(&mut Transform, &mut Sprite, &Tile), (Without<Player>, Without<Robot>)>,
) {
    if tile_size.is_changed() || tile_size.is_added() {
        let area_size = tile_size.0 * NUM_TILES as f32 + 2.0 * PADDING;
        for (mut t, mut sprite, tile) in tiles.iter_mut() {
            let pos_x: f32 =
                PADDING + tile_size.0 * tile.x as f32 - area_size * 0.5 + tile_size.0 * 0.5;
            let pos_y: f32 =
                PADDING + tile_size.0 * tile.y as f32 - area_size * 0.5 + tile_size.0 * 0.5;
            t.translation = Vec3::new(pos_x, pos_y, 0.);
            sprite.custom_size = Some(Vec2::new(tile_size.0, tile_size.0));
        }
    }
}

pub fn resize_actors(
    tile_size: Res<TileSize>,
    mut actors: Query<&mut Sprite, (Or<(With<Player>, With<Robot>)>, Without<Tile>)>,
) {
    if tile_size.is_changed() || tile_size.is_added() {
        let player_size = 0.9 * tile_size.0;
        for mut sprite in actors.iter_mut() {
            sprite.custom_size = Some(Vec2::new(player_size, player_size));
        }
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
    mut study_state: ResMut<StudyState>,
    mut commands: Commands,
    mut anim_timer: ResMut<AnimationTimer>,
    next_move: Option<ResMut<NextMove>>,
    mut state: ResMut<State<AppState>>,
    mut player: Query<(&Position, &mut NextPosition), (With<Player>, Without<Robot>)>,
    mut robot: Query<(&Position, &mut NextPosition), (With<Robot>, Without<Player>)>,
) {
    // we only apply the next move if the simulation is ready
    if *study_state != StudyState::Idle {
        return;
    }

    if let Some(mut human_move) = next_move {
        // animation timer
        *study_state = StudyState::Animation;
        anim_timer.0.reset();

        // robot move
        let (cur_pos_r, mut next_pos_r) = robot
            .get_single_mut()
            .expect("There should only be one player.");
        let robot_move = next_robot_move();
        *next_pos_r = next_pos_from_move(cur_pos_r, robot_move);

        // human move
        let (cur_pos_h, mut next_pos_h) = player
            .get_single_mut()
            .expect("There should only be one player.");
        *next_pos_h = next_pos_from_move(cur_pos_h, *human_move);

        if !is_move_legal(*human_move) {
            *human_move = NextMove::Idle;
        }

        // debug end game
        if *human_move == NextMove::Interact {
            state.set(AppState::End).expect("Could not change state.");
        }

        commands.remove_resource::<NextMove>();
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

fn next_robot_move() -> NextMove {
    NextMove::Idle
}

fn is_move_legal(next_move: NextMove) -> bool {
    true
}

pub fn draw_actor_to_pos(
    mut study_state: ResMut<StudyState>,
    time: Res<Time>,
    mut anim_timer: ResMut<AnimationTimer>,
    mut players: Query<
        (&mut Transform, &mut Position, &NextPosition),
        Or<(With<Robot>, With<Player>)>,
    >,
    tile_size: Res<TileSize>,
) {
    anim_timer.0.tick(time.delta());
    let mut t = anim_timer.0.elapsed().as_millis() as f32 / ANIM_DURATION.as_millis() as f32;
    t = t.clamp(0., 1.);
    t = t * t * (3. - 2. * t);

    let win_size = 2. * PADDING + NUM_TILES as f32 * tile_size.0;
    for (mut trans, mut pos, next_pos) in players.iter_mut() {
        let cur_x: f32 = PADDING + tile_size.0 * pos.x as f32 - win_size * 0.5 + tile_size.0 * 0.5;
        let cur_y: f32 = PADDING + tile_size.0 * pos.y as f32 - win_size * 0.5 + tile_size.0 * 0.5;

        let next_x: f32 =
            PADDING + tile_size.0 * next_pos.x as f32 - win_size * 0.5 + tile_size.0 * 0.5;
        let next_y: f32 =
            PADDING + tile_size.0 * next_pos.y as f32 - win_size * 0.5 + tile_size.0 * 0.5;

        let x = (1. - t) * cur_x + t * next_x;
        let y = (1. - t) * cur_y + t * next_y;
        trans.translation = Vec3::new(x, y, 1.);

        // if animation is over, we update position
        if anim_timer.0.finished() {
            pos.x = next_pos.x;
            pos.y = next_pos.y;
        }
    }

    // if animation is over, we reset animation state
    if anim_timer.0.finished() {
        *study_state = StudyState::Idle;
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

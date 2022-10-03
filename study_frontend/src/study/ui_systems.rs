use bevy::{prelude::*, window::WindowResized};

use crate::{assets::*, study::components::*};

use super::*;

pub fn setup_burger_ui(mut commands: Commands, menu_sprites: Res<MenuAssets>) {
    commands
        .spawn_bundle(SpriteBundle {
            texture: menu_sprites.sidebar_bg.clone(),
            ..default()
        })
        .insert(Study)
        .insert(BurgerUi)
        .add_children(|parent| {});
}

pub fn setup_adviser_ui(mut commands: Commands, menu_sprites: Res<MenuAssets>) {
    commands
        .spawn_bundle(SpriteBundle {
            texture: menu_sprites.sidebar_bg.clone(),
            ..default()
        })
        .insert(Study)
        .insert(AdviserUi)
        .add_children(|parent| {});
}

pub fn update_burger_ui() {}

pub fn scale_burger_ui(
    mut burger_ui: Query<(&mut Transform, &mut Sprite), (With<BurgerUi>, Without<AdviserUi>)>,
    window_size: Res<WindowSize>,
) {
    if window_size.is_changed() {
        let (mut transf, mut sprite) = burger_ui.single_mut();
        let x_pos = window_size.width * -0.5 + SIDEBAR_WIDTH * 0.5;
        transf.translation = Vec3::new(x_pos, 0., MENU_Z);
        sprite.custom_size = Some(Vec2::new(SIDEBAR_WIDTH, window_size.height));
    }
}

pub fn scale_adviser_ui(
    mut adviser_ui: Query<(&mut Transform, &mut Sprite), (With<AdviserUi>, Without<BurgerUi>)>,
    window_size: Res<WindowSize>,
) {
    if window_size.is_changed() {
        let (mut transf, mut sprite) = adviser_ui.single_mut();
        let x_pos = window_size.width * 0.5 - SIDEBAR_WIDTH * 0.5;
        transf.translation = Vec3::new(x_pos, 0., MENU_Z);
        sprite.custom_size = Some(Vec2::new(SIDEBAR_WIDTH, window_size.height));
    }
}

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
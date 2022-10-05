use bevy::{prelude::*, window::WindowResized};

use crate::{assets::*, study::components::*};

use super::*;

pub fn setup_burger_ui(
    mut commands: Commands,
    menu_sprites: Res<MenuAssets>,
    burger_components: Res<BurgerUiAssets>,
) {
    commands
        .spawn_bundle(SpriteBundle {
            texture: menu_sprites.sidebar_bg.clone(),
            ..default()
        })
        .insert(Study)
        .insert(BurgerUi)
        .add_children(|parent| {
            parent
                .spawn_bundle(SpriteBundle {
                    texture: burger_components.buns_inactive.clone(),
                    ..default()
                })
                .insert(BurgerComponent::Buns);
            parent
                .spawn_bundle(SpriteBundle {
                    texture: burger_components.patty_inactive.clone(),
                    ..default()
                })
                .insert(BurgerComponent::Patty);
            parent
                .spawn_bundle(SpriteBundle {
                    texture: burger_components.lettuce_inactive.clone(),
                    ..default()
                })
                .insert(BurgerComponent::Lettuce);
            parent
                .spawn_bundle(SpriteBundle {
                    texture: burger_components.tomato_inactive.clone(),
                    ..default()
                })
                .insert(BurgerComponent::Tomato);
            parent
                .spawn_bundle(SpriteBundle {
                    texture: burger_components.sauce_inactive.clone(),
                    ..default()
                })
                .insert(BurgerComponent::Sauce);
        });
}

pub fn setup_adviser_ui(mut commands: Commands, menu_sprites: Res<MenuAssets>) {
    commands
        .spawn_bundle(SpriteBundle {
            texture: menu_sprites.sidebar_bg.clone(),
            ..default()
        })
        .insert(Study)
        .insert(AdviserUi);
}

pub fn update_burger_ui(
    mut burger_components: Query<(&mut Handle<Image>, &BurgerComponent)>,
    progress: Res<BurgerProgress>,
    assets: Res<BurgerUiAssets>,
) {
    for (mut tex, bc) in burger_components.iter_mut() {
        match *bc {
            BurgerComponent::Buns => {
                *tex = if progress.buns {
                    assets.buns.clone()
                } else {
                    assets.buns_inactive.clone()
                };
            }
            BurgerComponent::Patty => {
                *tex = if progress.patty {
                    assets.patty.clone()
                } else {
                    assets.patty_inactive.clone()
                };
            }
            BurgerComponent::Lettuce => {
                *tex = if progress.lettuce {
                    assets.lettuce.clone()
                } else {
                    assets.lettuce_inactive.clone()
                };
            }
            BurgerComponent::Tomato => {
                *tex = if progress.tomato {
                    assets.tomato.clone()
                } else {
                    assets.tomato_inactive.clone()
                };
            }
            BurgerComponent::Sauce => {
                *tex = if progress.sauce {
                    assets.sauce.clone()
                } else {
                    assets.sauce_inactive.clone()
                };
            }
        }
    }
}

pub fn scale_burger_ui(
    mut burger_ui: Query<(&mut Transform, &mut Sprite), (With<BurgerUi>, Without<BurgerComponent>)>,
    mut burger_components: Query<
        (&mut Transform, &mut Sprite, &BurgerComponent),
        (With<BurgerComponent>, Without<BurgerUi>),
    >,
    window_size: Res<WindowSize>,
) {
    if window_size.is_changed() {
        let (mut transf, mut sprite) = burger_ui.single_mut();
        let x_pos = window_size.width * -0.5 + SIDEBAR_WIDTH * 0.5;
        transf.translation = Vec3::new(x_pos, 0., MENU_Z);
        sprite.custom_size = Some(Vec2::new(SIDEBAR_WIDTH, window_size.height));

        let fifth = window_size.height * 0.2;
        let window_upper = window_size.height * 0.5;
        let component_scale = SIDEBAR_WIDTH * 0.5;
        for (mut bc_transf, mut bc_sprite, bc) in burger_components.iter_mut() {
            bc_sprite.custom_size = Some(Vec2::new(component_scale, component_scale));
            match *bc {
                BurgerComponent::Buns => {
                    bc_transf.translation = Vec3::new(0., window_upper - 0.5 * fifth, MENU_Z + 1.0)
                }
                BurgerComponent::Patty => {
                    bc_transf.translation = Vec3::new(0., window_upper - 1.5 * fifth, MENU_Z + 1.0)
                }
                BurgerComponent::Lettuce => {
                    bc_transf.translation = Vec3::new(0., window_upper - 2.5 * fifth, MENU_Z + 1.0)
                }
                BurgerComponent::Tomato => {
                    bc_transf.translation = Vec3::new(0., window_upper - 3.5 * fifth, MENU_Z + 1.0)
                }
                BurgerComponent::Sauce => {
                    bc_transf.translation = Vec3::new(0., window_upper - 4.5 * fifth, MENU_Z + 1.0)
                }
            }
        }
    }
}

pub fn scale_adviser_ui(
    mut adviser_ui: Query<(&mut Transform, &mut Sprite), With<AdviserUi>>,
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
        (&mut Transform, &mut Position, &NextPosition, &Interact),
        Or<(With<Robot>, With<Player>)>,
    >,
    tile_size: Res<TileSize>,
) {
    anim_timer.0.tick(time.delta());
    let mut t = anim_timer.0.elapsed().as_millis() as f32 / ANIM_DURATION.as_millis() as f32;
    t = t.clamp(0., 1.);
    t = t * t * (3. - 2. * t);

    let win_size = 2. * PADDING + NUM_TILES as f32 * tile_size.0;
    for (mut trans, mut pos, next_pos, interact) in players.iter_mut() {
        let mut cur_x: f32 =
            PADDING + tile_size.0 * pos.x as f32 - win_size * 0.5 + tile_size.0 * 0.5;
        let mut cur_y: f32 =
            PADDING + tile_size.0 * pos.y as f32 - win_size * 0.5 + tile_size.0 * 0.5;

        let mut next_x: f32 =
            PADDING + tile_size.0 * next_pos.x as f32 - win_size * 0.5 + tile_size.0 * 0.5;
        let mut next_y: f32 =
            PADDING + tile_size.0 * next_pos.y as f32 - win_size * 0.5 + tile_size.0 * 0.5;

        // interact animation offset
        match *interact {
            Interact::No => (),
            Interact::In(interact_pos) => {
                let dx = interact_pos.x as i32 - pos.x as i32;
                let dy = interact_pos.y as i32 - pos.y as i32;
                next_x += tile_size.0 * 0.5 * dx as f32;
                next_y += tile_size.0 * 0.5 * dy as f32;
            }
            Interact::Out(interact_pos) => {
                let dx = interact_pos.x as i32 - pos.x as i32;
                let dy = interact_pos.y as i32 - pos.y as i32;
                cur_x += tile_size.0 * 0.5 * dx as f32;
                cur_y += tile_size.0 * 0.5 * dy as f32;
            }
        }

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

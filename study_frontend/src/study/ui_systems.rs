use std::f32::consts::PI;

use bevy::{prelude::*, window::WindowResized};
use study_shared_types::GameResults;

use crate::{assets::*, study::components::*};

use super::*;

pub fn setup_burger_ui(
    mut commands: Commands,
    menu_sprites: Res<MenuAssets>,
    burger_components: Res<BurgerUiAssets>,
    fonts: Res<FontAssets>,
) {
    commands
        .spawn_bundle(SpriteBundle {
            texture: menu_sprites.sidebar_bg.clone(),
            ..default()
        })
        .insert(Study)
        .insert(BurgerUi)
        .add_children(|parent| {
            // Human components text
            parent
                .spawn_bundle(Text2dBundle {
                    transform: Transform::from_xyz(0., 0., MENU_Z + 1.),
                    text: Text::from_section(
                        "You",
                        TextStyle {
                            font: fonts.default_font.clone(),
                            font_size: 90.0,
                            color: Color::BLACK,
                        },
                    )
                    .with_alignment(TextAlignment::CENTER),
                    ..default()
                })
                .insert(HumanBurgerText);
            // robot components text
            parent
                .spawn_bundle(Text2dBundle {
                    transform: Transform::from_xyz(0., 0., MENU_Z + 1.),
                    text: Text::from_section(
                        "Robot",
                        TextStyle {
                            font: fonts.default_font.clone(),
                            font_size: 90.0,
                            color: Color::BLACK,
                        },
                    )
                    .with_alignment(TextAlignment::CENTER),
                    ..default()
                })
                .insert(RobotBurgerText);
            // human components
            parent
                .spawn_bundle(SpriteBundle {
                    texture: burger_components.buns_inactive.clone(),
                    ..default()
                })
                .insert(BurgerComponent::Buns)
                .insert(HumanBurgerUi);
            parent
                .spawn_bundle(SpriteBundle {
                    texture: burger_components.patty_inactive.clone(),
                    ..default()
                })
                .insert(BurgerComponent::Patty)
                .insert(HumanBurgerUi);
            parent
                .spawn_bundle(SpriteBundle {
                    texture: burger_components.lettuce_inactive.clone(),
                    ..default()
                })
                .insert(BurgerComponent::Lettuce)
                .insert(HumanBurgerUi);
            parent
                .spawn_bundle(SpriteBundle {
                    texture: burger_components.tomato_inactive.clone(),
                    ..default()
                })
                .insert(BurgerComponent::Tomato)
                .insert(HumanBurgerUi);
            parent
                .spawn_bundle(SpriteBundle {
                    texture: burger_components.sauce_inactive.clone(),
                    ..default()
                })
                .insert(BurgerComponent::Sauce)
                .insert(HumanBurgerUi);

            // robot components
            parent
                .spawn_bundle(SpriteBundle {
                    texture: burger_components.buns_inactive.clone(),
                    ..default()
                })
                .insert(BurgerComponent::Buns)
                .insert(RobotBurgerUi);
            parent
                .spawn_bundle(SpriteBundle {
                    texture: burger_components.patty_inactive.clone(),
                    ..default()
                })
                .insert(BurgerComponent::Patty)
                .insert(RobotBurgerUi);
            parent
                .spawn_bundle(SpriteBundle {
                    texture: burger_components.lettuce_inactive.clone(),
                    ..default()
                })
                .insert(BurgerComponent::Lettuce)
                .insert(RobotBurgerUi);
            parent
                .spawn_bundle(SpriteBundle {
                    texture: burger_components.tomato_inactive.clone(),
                    ..default()
                })
                .insert(BurgerComponent::Tomato)
                .insert(RobotBurgerUi);
            parent
                .spawn_bundle(SpriteBundle {
                    texture: burger_components.sauce_inactive.clone(),
                    ..default()
                })
                .insert(BurgerComponent::Sauce)
                .insert(RobotBurgerUi);
        });
}

pub fn setup_adviser_ui(
    mut commands: Commands,
    menu_sprites: Res<MenuAssets>,
    fonts: Res<FontAssets>,
) {
    commands
        .spawn_bundle(SpriteBundle {
            texture: menu_sprites.sidebar_bg.clone(),
            ..default()
        })
        .insert(Study)
        .insert(AdviserUi)
        .add_children(|parent| {
            // timer text
            parent
                .spawn_bundle(Text2dBundle {
                    transform: Transform::from_xyz(0., 0., MENU_Z + 1.),
                    text: Text::from_section(
                        "TEST!",
                        TextStyle {
                            font: fonts.default_font.clone(),
                            font_size: 100.0,
                            color: Color::BLACK,
                        },
                    )
                    .with_alignment(TextAlignment::CENTER),
                    ..default()
                })
                .insert(TimerText);
            // burger score text
            parent
                .spawn_bundle(Text2dBundle {
                    transform: Transform::from_xyz(0., 0., MENU_Z + 1.),
                    text: Text::from_section(
                        "TEST!",
                        TextStyle {
                            font: fonts.default_font.clone(),
                            font_size: 45.0,
                            color: Color::BLACK,
                        },
                    )
                    .with_alignment(TextAlignment::CENTER),
                    ..default()
                })
                .insert(BurgerText);
        });
}

pub fn update_human_burger_ui(
    mut components_h: Query<
        (&mut Handle<Image>, &BurgerComponent),
        (With<HumanBurgerUi>, Without<RobotBurgerUi>),
    >,
    progress_query: Query<&BurgerProgress, (With<Player>, Without<Robot>)>,
    assets: Res<BurgerUiAssets>,
) {
    let progress = progress_query
        .get_single()
        .expect("There should only be one human.");

    for (mut tex, bc) in components_h.iter_mut() {
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

pub fn update_robot_burger_ui(
    mut components_r: Query<
        (&mut Handle<Image>, &BurgerComponent),
        (With<RobotBurgerUi>, Without<HumanBurgerUi>),
    >,
    progress_query: Query<&BurgerProgress, (With<Robot>, Without<Player>)>,
    assets: Res<BurgerUiAssets>,
) {
    let progress = progress_query
        .get_single()
        .expect("There should only be one human.");

    for (mut tex, bc) in components_r.iter_mut() {
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

pub fn scale_burger_ingredients_ui(
    mut human_components: Query<
        (&mut Transform, &mut Sprite, &BurgerComponent),
        (With<HumanBurgerUi>, Without<RobotBurgerUi>),
    >,
    mut robot_components: Query<
        (&mut Transform, &mut Sprite, &BurgerComponent),
        (With<RobotBurgerUi>, Without<HumanBurgerUi>),
    >,
    window_size: Res<WindowSize>,
) {
    if window_size.is_changed() {
        let window_upper = (window_size.height - SIDEBAR_PADDING) * 0.5;
        let component_scale = SIDEBAR_WIDTH * INGREDIENT_SCALE;
        let cx_h = 0.;
        let cy_h = window_upper * 0.5;
        let radius = SIDEBAR_WIDTH * 0.5 - component_scale * 0.5 - 10.;
        let theta = (2. * PI) / 5.;

        for (mut bc_transf, mut bc_sprite, bc) in human_components.iter_mut() {
            bc_sprite.custom_size = Some(Vec2::new(component_scale, component_scale));
            let count = match *bc {
                BurgerComponent::Buns => 0.,
                BurgerComponent::Patty => 1.,
                BurgerComponent::Lettuce => 2.,
                BurgerComponent::Tomato => 3.,
                BurgerComponent::Sauce => 4.,
            };
            let angle = count * theta;
            bc_transf.translation = Vec3::new(
                cx_h + radius * angle.cos() - 10.,
                cy_h + radius * angle.sin() - 15.,
                MENU_Z + 1.0,
            )
        }

        for (mut bc_transf, mut bc_sprite, bc) in robot_components.iter_mut() {
            bc_sprite.custom_size = Some(Vec2::new(component_scale, component_scale));
            let count = match *bc {
                BurgerComponent::Buns => 0.,
                BurgerComponent::Patty => 1.,
                BurgerComponent::Lettuce => 2.,
                BurgerComponent::Tomato => 3.,
                BurgerComponent::Sauce => 4.,
            };
            let angle = count * theta;
            bc_transf.translation = Vec3::new(
                cx_h + radius * angle.cos() - 10.,
                cy_h + radius * angle.sin() - window_upper - 40.,
                MENU_Z + 1.0,
            )
        }
    }
}

pub fn scale_burger_ui(
    mut burger_ui: Query<(&mut Transform, &mut Sprite), With<BurgerUi>>,
    window_size: Res<WindowSize>,
) {
    if window_size.is_changed() {
        let (mut transf, mut sprite) = burger_ui.single_mut();
        let x_pos = window_size.width * -0.5 + SIDEBAR_WIDTH * 0.5;
        transf.translation = Vec3::new(x_pos, 0., MENU_Z);
        sprite.custom_size = Some(Vec2::new(SIDEBAR_WIDTH, window_size.height));
    }
}

pub fn scale_burger_text_ui(
    mut human_text: Query<&mut Transform, (With<HumanBurgerText>, Without<RobotBurgerText>)>,
    mut robot_text: Query<&mut Transform, (With<RobotBurgerText>, Without<HumanBurgerText>)>,
    window_size: Res<WindowSize>,
) {
    if window_size.is_changed() {
        let human_text_pos = window_size.height * 0.5 - SIDEBAR_PADDING;
        let robot_text_pos = -SIDEBAR_PADDING;
        human_text.single_mut().translation = Vec3::new(0., human_text_pos, MENU_Z + 1.);
        robot_text.single_mut().translation = Vec3::new(0., robot_text_pos, MENU_Z + 1.);
    }
}

pub fn scale_adviser_ui(
    mut adviser_ui: Query<(&mut Transform, &mut Sprite), (With<AdviserUi>, Without<TimerText>)>,
    window_size: Res<WindowSize>,
) {
    if window_size.is_changed() {
        let (mut transf, mut sprite) = adviser_ui.single_mut();
        let x_pos = window_size.width * 0.5 - SIDEBAR_WIDTH * 0.5;
        transf.translation = Vec3::new(x_pos, 0., MENU_Z);
        sprite.custom_size = Some(Vec2::new(SIDEBAR_WIDTH, window_size.height));
    }
}

pub fn scale_adviser_text_ui(
    mut timer_text: Query<&mut Transform, (With<TimerText>, Without<BurgerText>)>,
    mut burger_text: Query<&mut Transform, (With<BurgerText>, Without<TimerText>)>,
    window_size: Res<WindowSize>,
) {
    if window_size.is_changed() {
        let text_pos = window_size.height * 0.5 - SIDEBAR_PADDING;
        timer_text.single_mut().translation = Vec3::new(0., text_pos, MENU_Z + 1.);
        burger_text.single_mut().translation = Vec3::new(0., text_pos - 100., MENU_Z + 1.);
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
        let new_tile_size = (size_min - (TILE_PADDING * 2.0)) / NUM_TILES as f32;
        tile_size.0 = new_tile_size;
        window_size.width = e.width;
        window_size.height = e.height;
    }
}

pub fn update_timer_text(mut timer_text: Query<&mut Text, With<TimerText>>, timer: Res<GameTimer>) {
    let mut text = timer_text.single_mut();
    let remaining = GAME_DURATION.saturating_sub(timer.0.elapsed());
    let minutes_left = remaining.as_secs() / 60;
    let seconds_left = remaining.as_secs() % 60;
    text.sections[0].value = format!("{}:{:02}", minutes_left, seconds_left);
}

pub fn update_burger_text(
    mut burger_text: Query<&mut Text, With<BurgerText>>,
    game_results: Res<GameResults>,
) {
    let mut text = burger_text.single_mut();
    text.sections[0].value = format!(
        "Burgers Made:\n{}",
        game_results.human_burgers + game_results.robot_burgers
    );
}

pub fn resize_tiles(
    tile_size: Res<TileSize>,
    mut tiles: Query<(&mut Transform, &mut Sprite, &Tile), (Without<Player>, Without<Robot>)>,
) {
    if tile_size.is_changed() || tile_size.is_added() {
        let area_size = tile_size.0 * NUM_TILES as f32 + 2.0 * TILE_PADDING;
        for (mut t, mut sprite, tile) in tiles.iter_mut() {
            let pos_x: f32 =
                TILE_PADDING + tile_size.0 * tile.x as f32 - area_size * 0.5 + tile_size.0 * 0.5;
            let pos_y: f32 =
                TILE_PADDING + tile_size.0 * tile.y as f32 - area_size * 0.5 + tile_size.0 * 0.5;
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
        let player_size = 0.7 * tile_size.0;
        for mut sprite in actors.iter_mut() {
            sprite.custom_size = Some(Vec2::new(player_size, player_size));
        }
    }
}

pub fn resize_speech_bubble(
    tile_size: Res<TileSize>,
    mut bubble: Query<(&mut Transform, &mut Sprite), With<SpeechBubble>>,
) {
    if tile_size.is_changed() || tile_size.is_added() {
        let player_size = 0.7 * tile_size.0;
        let (mut transf, mut sprite) = bubble.single_mut();
        transf.translation = Vec3::new(player_size * 1.5, -player_size * 0.9, SPEECH_BUBBLE_Z);
        sprite.custom_size = Some(Vec2::new(player_size * 3., player_size));
    }
}

pub fn toggle_speech_bubble(
    mut bubble: Query<&mut Visibility, With<SpeechBubble>>,
    active_advisers: Res<ActiveAdvisers>,
) {
    if active_advisers.is_changed() {
        bubble.single_mut().is_visible = !active_advisers.is_empty();
    }
}

pub fn update_adviser_ui(
    bubble: Query<Entity, With<SpeechBubble>>,
    active_advisers: Res<ActiveAdvisers>,
    mut commands: Commands,
    adviser_icons: Res<AdviserAssets>,
    tile_size: Res<TileSize>,
    synth_game: Res<SynthGame>,
) {
    if active_advisers.is_changed() {
        let bubble_id = bubble.single();
        commands.entity(bubble_id).add_children(|parent| {
            let mut adviser_pos_y = 0.;
            // safety
            for saf_adv in &active_advisers.safety {
                let mut adviser_pos_x = -0.5 * tile_size.0;
                parent
                    .spawn_bundle(SpriteBundle {
                        sprite: Sprite {
                            custom_size: Some(Vec2::new(ADVISER_SIZE, ADVISER_SIZE)),
                            ..default()
                        },
                        transform: Transform::from_xyz(adviser_pos_x, adviser_pos_y, MENU_Z + 1.),
                        texture: adviser_icons.person.clone(),
                        ..default()
                    })
                    .insert(Study)
                    .insert(AdviserIcon);
                adviser_pos_x += ADVISER_SIZE;
                parent
                    .spawn_bundle(SpriteBundle {
                        sprite: Sprite {
                            custom_size: Some(Vec2::new(ADVISER_SIZE, ADVISER_SIZE)),
                            ..default()
                        },
                        transform: Transform::from_xyz(adviser_pos_x, adviser_pos_y, MENU_Z + 1.),
                        texture: adviser_icons.cross.clone(),
                        ..default()
                    })
                    .insert(Study)
                    .insert(AdviserIcon);
                adviser_pos_x += ADVISER_SIZE;
                for (i, c) in saf_adv.chars().enumerate() {
                    if c == 'X' {
                        continue;
                    }
                    if c == '0' {
                        panic!("Safety adviser with negative proposition, not supported!")
                    }
                    let sprite_handle = match synth_game.graph.human_ap[i].as_str() {
                        "buns_h" => adviser_icons.buns.clone(),
                        "patty_h" => adviser_icons.patty.clone(),
                        "tomato_h" => adviser_icons.tomato.clone(),
                        "lettuce_h" => adviser_icons.lettuce.clone(),
                        "ketchup_h" => adviser_icons.sauce.clone(),
                        _ => panic!("Could not find sprite!"),
                    };
                    parent
                        .spawn_bundle(SpriteBundle {
                            sprite: Sprite {
                                custom_size: Some(Vec2::new(ADVISER_SIZE, ADVISER_SIZE)),
                                ..default()
                            },
                            transform: Transform::from_xyz(
                                adviser_pos_x,
                                adviser_pos_y,
                                MENU_Z + 1.,
                            ),
                            texture: sprite_handle,
                            ..default()
                        })
                        .insert(Study)
                        .insert(AdviserIcon);
                    adviser_pos_x += ADVISER_SIZE;
                }
                adviser_pos_y -= ADVISER_SIZE;
            }

            // fairness
            for fair_adv in &active_advisers.fairness {
                let mut adviser_pos_x = -0.5 * tile_size.0;
                parent
                    .spawn_bundle(SpriteBundle {
                        sprite: Sprite {
                            custom_size: Some(Vec2::new(ADVISER_SIZE, ADVISER_SIZE)),
                            ..default()
                        },
                        transform: Transform::from_xyz(adviser_pos_x, adviser_pos_y, MENU_Z + 1.),
                        texture: adviser_icons.person.clone(),
                        ..default()
                    })
                    .insert(Study)
                    .insert(AdviserIcon);
                adviser_pos_x += ADVISER_SIZE;
                parent
                    .spawn_bundle(SpriteBundle {
                        sprite: Sprite {
                            custom_size: Some(Vec2::new(ADVISER_SIZE, ADVISER_SIZE)),
                            ..default()
                        },
                        transform: Transform::from_xyz(adviser_pos_x, adviser_pos_y, MENU_Z + 1.),
                        texture: adviser_icons.arrow.clone(),
                        ..default()
                    })
                    .insert(Study)
                    .insert(AdviserIcon);
                adviser_pos_x += ADVISER_SIZE;
                for (i, c) in fair_adv.chars().enumerate() {
                    if c == 'X' {
                        continue;
                    }
                    if c == '0' {
                        panic!("fairness adviser with negative proposition, not supported!")
                    }
                    let sprite_handle = match synth_game.graph.human_ap[i].as_str() {
                        "buns_h" => adviser_icons.buns.clone(),
                        "patty_h" => adviser_icons.patty.clone(),
                        "tomato_h" => adviser_icons.tomato.clone(),
                        "lettuce_h" => adviser_icons.lettuce.clone(),
                        "ketchup_h" => adviser_icons.sauce.clone(),
                        _ => panic!("Could not find sprite!"),
                    };
                    parent
                        .spawn_bundle(SpriteBundle {
                            sprite: Sprite {
                                custom_size: Some(Vec2::new(ADVISER_SIZE, ADVISER_SIZE)),
                                ..default()
                            },
                            transform: Transform::from_xyz(
                                adviser_pos_x,
                                adviser_pos_y,
                                MENU_Z + 1.,
                            ),
                            texture: sprite_handle,
                            ..default()
                        })
                        .insert(Study)
                        .insert(AdviserIcon);
                    adviser_pos_x += ADVISER_SIZE;
                }
                adviser_pos_y -= ADVISER_SIZE;
            }
        });
    }
}

pub fn draw_actor_to_pos(
    anim_timer: Res<AnimationTimer>,
    mut players: Query<
        (&mut Transform, &mut Position, &NextPosition, &Interact),
        Or<(With<Robot>, With<Player>)>,
    >,
    tile_size: Res<TileSize>,
) {
    let mut t = anim_timer.0.elapsed().as_millis() as f32 / ANIM_DURATION.as_millis() as f32;
    t = t.clamp(0., 1.);
    t = t * t * (3. - 2. * t);

    let win_size = 2. * TILE_PADDING + NUM_TILES as f32 * tile_size.0;
    for (mut trans, mut pos, next_pos, interact) in players.iter_mut() {
        let mut cur_x: f32 =
            TILE_PADDING + tile_size.0 * pos.x as f32 - win_size * 0.5 + tile_size.0 * 0.5;
        let mut cur_y: f32 =
            TILE_PADDING + tile_size.0 * pos.y as f32 - win_size * 0.5 + tile_size.0 * 0.5;

        let mut next_x: f32 =
            TILE_PADDING + tile_size.0 * next_pos.x as f32 - win_size * 0.5 + tile_size.0 * 0.5;
        let mut next_y: f32 =
            TILE_PADDING + tile_size.0 * next_pos.y as f32 - win_size * 0.5 + tile_size.0 * 0.5;

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
}

pub fn update_fade_away_sprite(
    anim_timer: Res<AnimationTimer>,
    is_violated: Option<Res<SafetyViolated>>,
    mut sprite: Query<&mut Sprite, With<FadeAwayScreen>>,
    window_size: ResMut<WindowSize>,
) {
    let mut sprite = sprite.single_mut();
    if let Some(_) = is_violated {
        let a = anim_timer.0.elapsed().as_millis() as f32 / ANIM_DURATION.as_millis() as f32;
        sprite.custom_size = Some(Vec2::new(window_size.width, window_size.height));
        sprite.color.set_a(a);
    } else {
        sprite.color.set_a(0.0);
    }
}

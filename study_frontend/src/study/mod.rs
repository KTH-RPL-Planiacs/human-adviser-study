use std::time::Duration;

pub mod components;
pub mod logic_systems;
pub mod ui_systems;

pub const TILE_PADDING: f32 = 60.0;
pub const NUM_TILES: usize = 5;

pub const MENU_Z: f32 = 10.0;
pub const SIDEBAR_WIDTH: f32 = 300.0;
pub const INGREDIENT_SCALE: f32 = 0.3;
pub const SIDEBAR_PADDING: f32 = 50.0;
pub const SPEECH_BUBBLE_Z: f32 = MENU_Z + 10.;
pub const ADVISER_SIZE: f32 = 75.0;

pub const ANIM_DURATION: Duration = Duration::from_millis(400);
pub const FADE_DURATION: Duration = Duration::from_millis(1000);
pub const GAME_DURATION: Duration = Duration::from_secs(90);

pub const DELIVERY_POS_H: (usize, usize) = (2, 4);
pub const BUNS_POS_H: (usize, usize) = (1, 3);
pub const LETTUCE_POS_H: (usize, usize) = (4, 3);
pub const PATTY_POS_H: (usize, usize) = (0, 3);
pub const TOMATO_POS_H: (usize, usize) = (2, 3);
pub const SAUCE_POS_H: (usize, usize) = (3, 3);

pub const DELIVERY_POS_R: (usize, usize) = (2, 0);
pub const BUNS_POS_R: (usize, usize) = (1, 1);
pub const LETTUCE_POS_R: (usize, usize) = (4, 1);
pub const PATTY_POS_R: (usize, usize) = (0, 1);
pub const TOMATO_POS_R: (usize, usize) = (2, 1);
pub const SAUCE_POS_R: (usize, usize) = (3, 1);

pub const HUMAN_START: (usize, usize) = (2, 4);
pub const ROBOT_START: (usize, usize) = (2, 0);

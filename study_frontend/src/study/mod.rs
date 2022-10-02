use std::time::Duration;

pub mod components;
pub mod logic_systems;
pub mod ui_systems;

pub const PADDING: f32 = 10.0;
pub const NUM_TILES: usize = 5;

pub const MENU_Z: f32 = 10.0;
pub const SIDEBAR_WIDTH: f32 = 300.0;

pub const ANIM_DURATION: Duration = Duration::from_millis(500);

pub const DELIVERY_POS_H: (usize, usize) = (2, 4);
pub const BUNS_POS_H: (usize, usize) = (1, 3);
pub const LETTUCE_POS_H: (usize, usize) = (4, 3);
pub const PATTY_POS_H: (usize, usize) = (0, 3);
pub const TOMATO_POS_H: (usize, usize) = (2, 3);
pub const SAUCE_POS_H: (usize, usize) = (3, 3);

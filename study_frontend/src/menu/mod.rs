use bevy::prelude::*;

pub mod end;
pub mod main;

pub const DISABLED_BUTTON: Color = Color::rgb(0.8, 0.5, 0.5);
pub const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
pub const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
pub const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);
pub const BUTTON_TEXT: Color = Color::rgb(0.9, 0.9, 0.9);

pub const PART_ID_LEN: usize = 4;

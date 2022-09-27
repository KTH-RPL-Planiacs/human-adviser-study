use std::time::Duration;

pub mod components;
pub mod systems;

pub const PADDING: f32 = 10.0;
pub const NUM_TILES: usize = 5;

pub const ANIM_DURATION: Duration = Duration::from_millis(500);

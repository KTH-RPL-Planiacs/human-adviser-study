use bevy::prelude::*;

#[derive(Component)]
pub struct Study;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Robot;

#[derive(Debug, Copy, Clone)]
pub enum NextMove {
    Up,
    Down,
    Left,
    Right,
    Interact,
}

#[derive(Debug)]
pub struct TileSize(pub f32);

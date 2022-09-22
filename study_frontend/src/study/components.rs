use bevy::prelude::*;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum StudyState {
    Idle,
    Animation,
}

#[derive(Component)]
pub struct Study;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Robot;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum NextMove {
    Idle,
    Up,
    Down,
    Left,
    Right,
    Interact,
}

#[derive(Component, Debug, Copy, Clone)]
pub struct Tile {
    pub x: usize,
    pub y: usize,
}

#[derive(Component, Debug, Copy, Clone)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

#[derive(Component, Debug, Copy, Clone)]
pub struct NextPosition {
    pub x: usize,
    pub y: usize,
}

#[derive(Component, Debug, Copy, Clone)]
pub enum TileType {
    Floor,
}

#[derive(Debug, Copy, Clone)]
pub struct TileSize(pub f32);

#[derive(Default)]
pub struct AnimationTimer(pub Timer);

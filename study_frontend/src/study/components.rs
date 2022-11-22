use std::{error::Error, fmt::Display, str::FromStr, usize};

use bevy::prelude::*;

use crate::assets::Guards;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum StudyState {
    Idle,
    Animation,
    FadeAway,
}

#[derive(Component)]
pub struct Study;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Robot;

#[derive(Component)]
pub struct BurgerUi;

#[derive(Component)]
pub enum BurgerComponent {
    Buns,
    Patty,
    Lettuce,
    Tomato,
    Sauce,
}

#[derive(Component, Debug)]
pub enum Interact {
    No,
    In(Position),
    Out(Position),
}

#[derive(Component)]
pub struct AdviserUi;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum NextMove {
    Idle,
    Up,
    Down,
    Left,
    Right,
    Interact,
}

impl FromStr for NextMove {
    type Err = ParseMoveError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "up" => Ok(NextMove::Up),
            "down" => Ok(NextMove::Down),
            "left" => Ok(NextMove::Left),
            "right" => Ok(NextMove::Right),
            "interact" => Ok(NextMove::Interact),
            _ => Err(ParseMoveError),
        }
    }
}

#[derive(Debug)]
pub struct ParseMoveError;
impl Display for ParseMoveError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Something went wrong parsing a Move!")
    }
}
impl Error for ParseMoveError {}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct HumanNextMove(pub NextMove);

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct RobotNextMove(pub NextMove);

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

impl Position {
    pub fn is_equal(&self, coords: (usize, usize)) -> bool {
        self.x == coords.0 && self.y == coords.1
    }
}

#[derive(Component, Debug, Copy, Clone)]
pub struct NextPosition {
    pub x: usize,
    pub y: usize,
}

#[derive(Component, Debug, Copy, Clone)]
pub enum TileType {
    Default,
    Floor,
    Buns,
    Patty,
    Lettuce,
    Tomato,
    Sauce,
}

#[derive(Debug, Copy, Clone)]
pub struct TileSize(pub f32);

#[derive(Debug, Copy, Clone)]
pub struct WindowSize {
    pub width: f32,
    pub height: f32,
}

#[derive(Default)]
pub struct AnimationTimer(pub Timer);

#[derive(Default)]
pub struct GameTimer(pub Timer);

#[derive(Component)]
pub struct TimerText;

#[derive(Default, Debug, Copy, Clone)]
pub struct BurgerProgress {
    pub assembled: u32,
    pub buns: bool,
    pub patty: bool,
    pub lettuce: bool,
    pub tomato: bool,
    pub sauce: bool,
}

impl BurgerProgress {
    fn ready(&self) -> bool {
        self.buns && self.patty && self.lettuce && self.tomato && self.sauce
    }

    pub fn make_burger(&mut self) {
        if self.ready() {
            self.assembled += 1;
            self.buns = false;
            self.patty = false;
            self.lettuce = false;
            self.tomato = false;
            self.sauce = false;
        }
    }

    pub fn reset(&mut self) {
        self.buns = false;
        self.patty = false;
        self.lettuce = false;
        self.tomato = false;
        self.sauce = false;
    }
}

#[derive(Default, Debug)]
pub struct ActiveAdvisers {
    pub safety: Vec<Guards>,
    pub fairness: Vec<Guards>,
}

impl ActiveAdvisers {
    pub fn clear_all(&mut self) {
        self.safety.clear();
        self.fairness.clear();
    }

    pub fn safety_violated(&self) -> bool {
        false
    }
}

pub struct SafetyViolated;

#[derive(Component)]
pub struct FadeAwayScreen;

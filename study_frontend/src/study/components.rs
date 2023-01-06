use std::{error::Error, fmt::Display, str::FromStr, usize};

use bevy::prelude::*;

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
pub struct HumanBurgerUi;

#[derive(Component)]
pub struct RobotBurgerUi;

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

#[derive(Component)]
pub struct AdviserIcon;

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

impl NextPosition {
    pub fn as_pos(&self) -> Position {
        Position {
            x: self.x,
            y: self.y,
        }
    }
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

#[derive(Component)]
pub struct BurgerText;

#[derive(Default, Debug, Copy, Clone, Component)]
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

    pub fn make_burger(&mut self) -> bool {
        if self.ready() {
            self.assembled += 1;
            self.buns = false;
            self.patty = false;
            self.lettuce = false;
            self.tomato = false;
            self.sauce = false;
            return true;
        }
        return false;
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
    pub safety: Vec<String>,
    pub fairness: Vec<String>,
}

impl ActiveAdvisers {
    pub fn clear_all(&mut self) {
        self.safety.clear();
        self.fairness.clear();
    }

    pub fn safety_violated(&self, obs: &String) -> bool {
        for guard in &self.safety {
            if obs_match_guard(obs, guard) {
                return true;
            }
        }
        false
    }
}

fn obs_match_guard(obs: &String, guard: &String) -> bool {
    assert_eq!(obs.len(), guard.len());
    for (i, c) in guard.chars().enumerate() {
        if c == 'X' {
            continue;
        }
        if c != obs
            .chars()
            .nth(i)
            .expect("obs_match_guard: obs and guard should be same length!")
        {
            return false;
        }
    }
    true
}

pub struct SafetyViolated;

#[derive(Component)]
pub struct FadeAwayScreen;

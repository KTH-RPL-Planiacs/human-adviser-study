use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct GameResults {
    pub participant_id: i32,
    pub adviser_mode: u32,
    pub steps_taken: u32,
    pub safety_violated: u32,
    pub human_burgers: u32,
    pub robot_burgers: u32,
}

impl GameResults {
    pub fn to_json(&self) -> String {
        format!("{{\"participant_id\": {}, \"adviser_mode\": {}, \"steps_taken\": {}, \"safety_violated\": {}, \"human_burgers\": {}, \"robot_burgers\": {}}}", self.participant_id, self.adviser_mode, self.steps_taken, self.safety_violated, self.human_burgers, self.robot_burgers)
    }
}

#[derive(Copy, Clone, Debug, Deserialize, Serialize)]
pub enum AdviserMode {
    LeastLimiting,
    NextMove,
    None,
}

impl ToString for AdviserMode {
    fn to_string(&self) -> String {
        match self {
            AdviserMode::LeastLimiting => "LeastLimiting".to_string(),
            AdviserMode::NextMove => "NextMove".to_string(),
            AdviserMode::None => "None".to_string(),
        }
    }
}

impl AdviserMode {
    pub fn to_num(&self) -> u32 {
        match self {
            AdviserMode::LeastLimiting => 0,
            AdviserMode::NextMove => 1,
            AdviserMode::None => 2,
        }
    }

    pub fn from_num(n: u32) -> Self {
        match n {
            0 => AdviserMode::LeastLimiting,
            1 => AdviserMode::NextMove,
            2 => AdviserMode::None,
            _ => panic!("invalid number!"),
        }
    }
}

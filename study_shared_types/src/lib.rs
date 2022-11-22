use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct GameResults {
    pub participant_id: i32,
    pub human_burgers: u32,
    pub robot_burgers: u32,
}

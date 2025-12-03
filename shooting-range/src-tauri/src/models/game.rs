use serde::{Deserialize, Serialize};
use crate::models::target::Target;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Game {
    pub id: Option<i32>,
    pub name: String,
    pub total_time: i32,
    pub targets: Vec<GameTarget>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameTarget {
    pub id: Option<i32>,
    pub target: Target,
    pub start_time: i32,
    pub end_time: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameTargetInput {
    pub target_id: i32,
    pub start_time: i32,
    pub end_time: i32,
}


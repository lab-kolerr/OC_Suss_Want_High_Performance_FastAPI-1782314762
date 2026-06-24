use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Behavior {
    pub id: i32,
    pub agent_id: i32,
    pub action: String,
}
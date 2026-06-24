use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AIAgent {
    pub id: i32,
    pub name: String,
    pub description: String,
}
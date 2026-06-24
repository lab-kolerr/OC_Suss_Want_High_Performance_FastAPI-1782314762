use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SystemLog {
    pub id: i32,
    pub message: String,
    pub timestamp: String,
}
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SecurityEvent {
    pub id: i32,
    pub event_type: String,
    pub timestamp: String,
}
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DetectionRule {
    pub id: i32,
    pub rule: String,
}
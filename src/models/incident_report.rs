use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct IncidentReport {
    pub id: i32,
    pub description: String,
    pub timestamp: String,
}
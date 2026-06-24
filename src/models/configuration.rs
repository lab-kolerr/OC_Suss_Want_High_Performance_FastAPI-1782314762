use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Configuration {
    pub id: i32,
    pub setting: String,
    pub value: String,
}
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Alert {
    pub id: i32,
    pub user_id: i32,
    pub message: String,
}
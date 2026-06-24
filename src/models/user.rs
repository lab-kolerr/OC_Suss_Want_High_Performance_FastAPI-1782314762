use bcrypt::{hash, verify};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
}

impl User {
    pub fn new(username: String, password: String) -> Self {
        User { id: 0, username, password }
    }

    pub async fn save(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.password = hash(&self.password, 10)?;
        // Save user to database logic here
        Ok(())
    }
}
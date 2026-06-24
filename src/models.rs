/// Represents an item in the Auto application.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Item {
    pub id: u32,
    pub name: String,
}

/// Validates an item before processing.
pub fn validate_item(item: &Item) -> Result<(), String> {
    if item.name.is_empty() {
        return Err("Item name cannot be empty".to_string());
    }
    Ok(())
}
pub mod utils;

pub fn process_data(data: &str) -> String {
    data.trim().to_uppercase()
}
pub fn sanitize_input(input: &str) -> String {
    input.replace("<", "&lt;").replace(">", "&gt;")
}

pub fn validate_password(password: &str) -> bool {
    password.len() >= 8 && password.chars().any(|c| c.is_digit(10))
}
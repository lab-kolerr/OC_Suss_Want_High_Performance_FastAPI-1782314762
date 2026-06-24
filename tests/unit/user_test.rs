use super::*;
use validator::Validate;

#[test]
fn test_user_validation() {
    let user = User {
        id: 1,
        email: "test@example.com".to_string(),
        hashed_password: "hashed_password".to_string(),
    };
    assert!(user.validate().is_ok());
}

#[test]
fn test_user_invalid_email() {
    let user = User {
        id: 1,
        email: "invalid_email".to_string(),
        hashed_password: "hashed_password".to_string(),
    };
    assert!(user.validate().is_err());
}
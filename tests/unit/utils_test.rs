use super::*;

#[test]
fn test_hash_password() {
    let password = "my_password";
    let hashed = hash_password(password);
    assert!(verify_password(password, &hashed));
}

#[test]
fn test_invalid_hash() {
    let password = "my_password";
    let invalid_hash = "invalid_hash";
    assert!(!verify_password(password, invalid_hash));
}
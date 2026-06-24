#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_creation() {
        let user = User { id: 1, email: String::from("test@example.com"), hashed_password: String::from("hashed_password") };
        assert_eq!(user.email, "test@example.com");
    }
    
    #[tokio::test]
    async fn test_async_endpoint() {
        let response = get_users().await;
        assert!(response.is_ok());
    }
}
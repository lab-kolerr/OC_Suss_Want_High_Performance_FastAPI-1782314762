#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_loading() {
        let config = load_config();
        assert_eq!(config.database_url, "postgres://user:password@localhost/db");
    }
}
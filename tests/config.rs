#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use rust_server::settings::get_config;

    #[test]
    fn test_get_config_success() {
        let config = get_config("tests/assets/setup.toml").unwrap();
        assert_eq!(config.server.host, "localhost");
        assert_eq!(config.server.port, "8080");
        assert_eq!(config.db.username, "user");
        assert_eq!(config.db.port, 5432);
        assert_eq!(config.db.host, "postgres");
        assert_eq!(config.db.database_name, "my_database");
    }

    #[test]
    fn test_get_config_missing_file() {
        let result = get_config("");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("not found"));
    }

}
use infinity_config::{
    ConfigError,
    config::{AppConfig, AppInfo, ServerConfig},
};

#[test]
fn app_info_from_toml() {
    let config: AppInfo = toml::from_str(r#"name = "test""#).unwrap();
    assert_eq!(config.name, "test");
    assert_eq!(config.version, None);
}

#[test]
fn server_config_from_toml() {
    let config: ServerConfig = toml::from_str(
        r#"
host = "127.0.0.1"
port = 9090
"#,
    )
    .unwrap();

    assert_eq!(config.host, "127.0.0.1");
    assert_eq!(config.port, 9090);
}

#[test]
fn full_app_config_from_toml() {
    let config: AppConfig = toml::from_str(
        r#"
[app]
name = "Infinity"

[server]
host = "0.0.0.0"
port = 3000

[database]
url = "postgres://localhost/db"
max_connections = 10

[redis]
host = "127.0.0.1"
port = 6379

[ai]
provider = "openai"
api_key = "sk-test"
"#,
    )
    .unwrap();

    assert_eq!(config.app.name, "Infinity");
    assert_eq!(config.server.port, 3000);
    assert_eq!(config.database.url, "postgres://localhost/db");
    assert_eq!(config.redis.host, "127.0.0.1");
    assert_eq!(config.ai.provider, "openai");
    assert_eq!(config.ai.api_key.as_deref(), Some("sk-test"));
}

#[test]
fn toml_parse_error() {
    let result = toml::from_str::<AppConfig>(
        r#"
[app]
name = "test"

[server]
host = "0.0.0.0"
port = "not-a-number"
"#,
    );

    assert!(result.is_err());
}

#[test]
fn toml_missing_sections_use_defaults() {
    let config = toml::from_str::<AppConfig>(
        r#"
[server]
host = "0.0.0.0"
port = 8080
"#,
    )
    .unwrap();

    assert_eq!(config.app.name, "Infinity");
    assert_eq!(config.database.max_connections, 10);
    assert_eq!(config.redis.port, 6379);
}

#[test]
fn toml_roundtrip() {
    let config: ServerConfig = toml::from_str(
        r#"
host = "0.0.0.0"
port = 8080
"#,
    )
    .unwrap();
    let serialized = toml::to_string(&config).unwrap();
    let restored: ServerConfig = toml::from_str(&serialized).unwrap();

    assert_eq!(config.host, restored.host);
    assert_eq!(config.port, restored.port);
}

#[test]
fn config_error_display() {
    let err = ConfigError::file_not_found("/tmp/missing.toml");
    assert_eq!(
        err.to_string(),
        "configuration file not found: /tmp/missing.toml"
    );

    let err = ConfigError::missing_field("app.name");
    assert_eq!(err.to_string(), "missing configuration field: app.name");
}

#[test]
fn error_conversions() {
    let io_err = std::io::Error::from(std::io::ErrorKind::NotFound);
    let config_err: ConfigError = io_err.into();
    assert!(matches!(config_err, ConfigError::FileNotFound(_)));

    let io_err = std::io::Error::from(std::io::ErrorKind::PermissionDenied);
    let config_err: ConfigError = io_err.into();
    assert!(matches!(config_err, ConfigError::FileReadError(_)));

    let json_err = serde_json::from_str::<serde_json::Value>("invalid json").unwrap_err();
    let config_err: ConfigError = json_err.into();
    assert!(matches!(config_err, ConfigError::ParseError(_)));

    let yaml_err = serde_yaml::from_str::<serde_json::Value>(": invalid").unwrap_err();
    let config_err: ConfigError = yaml_err.into();
    assert!(matches!(config_err, ConfigError::ParseError(_)));
}

#[test]
fn config_error_roundtrip() {
    let original = ConfigError::ValidationError("port must be valid".to_string());
    let json = serde_json::to_string(&original).unwrap();
    let restored: ConfigError = serde_json::from_str(&json).unwrap();

    assert_eq!(original.to_string(), restored.to_string());
}

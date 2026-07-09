use infinity_config::{
    ConfigError, ConfigManager, Environment, Validate, config::AppConfig, loader,
};

fn workspace_configs_dir() -> &'static str {
    concat!(env!("CARGO_MANIFEST_DIR"), "/../../apps/admin/configs")
}

#[test]
fn loads_base_application_toml() {
    let config_path = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../apps/admin/configs/application.toml"
    );
    let config: toml::Value = loader::load_file(config_path).unwrap();

    assert_eq!(config["app"]["name"].as_str().unwrap(), "Infinity");
    assert_eq!(config["server"]["host"].as_str().unwrap(), "0.0.0.0");
    assert_eq!(config["server"]["port"].as_integer().unwrap(), 8080);
}

#[test]
fn layered_dev_config_overrides_database_url() {
    let config =
        ConfigManager::load_from_dir_with_env(workspace_configs_dir(), Environment::Dev).unwrap();

    assert_eq!(config.app.name, "Infinity");
    assert_eq!(config.server.port, 8080);
    assert_eq!(
        config.database.url,
        "postgres://postgres:postgres@localhost:30432/infinity"
    );
    assert_eq!(config.database.max_connections, 10);
    assert_eq!(config.logger.unwrap().level, "debug");
}

#[test]
fn layered_prod_config_overrides_nested_values() {
    let config =
        ConfigManager::load_from_dir_with_env(workspace_configs_dir(), Environment::Prod).unwrap();

    assert_eq!(config.server.host, "0.0.0.0");
    assert_eq!(config.server.port, 80);
    assert_eq!(config.database.max_connections, 50);
    assert!(config.logger.unwrap().json);
}

#[test]
fn missing_base_file_returns_error() {
    let result: infinity_config::Result<AppConfig> =
        loader::load_layered_toml("missing-application.toml", None);

    assert!(matches!(result, Err(ConfigError::FileNotFound(_))));
}

#[test]
fn invalid_toml_returns_parse_error() {
    let result: infinity_config::Result<toml::Value> = loader::parse_str(
        r#"
[app]
name = "test"
[server
host = "0.0.0.0"
"#,
        loader::Format::Toml,
    );

    assert!(matches!(result, Err(ConfigError::ParseError(_))));
}

#[test]
fn validation_checks_all_sections() {
    let invalid = r#"
[app]
name = "Infinity"

[server]
host = "0.0.0.0"
port = 8080

[database]
url = ""
max_connections = 10

[redis]
host = "127.0.0.1"
port = 6379

[ai]
provider = "openai"
"#;

    let config: AppConfig = toml::from_str(invalid).unwrap();
    let result = config.validate();

    assert!(matches!(result, Err(ConfigError::ValidationError(_))));
}

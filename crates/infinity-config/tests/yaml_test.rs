use infinity_config::config::{
    AiConfig, AppConfig, AppInfo, DatabaseConfig, RedisConfig, ServerConfig,
};

#[test]
fn app_info_from_yaml() {
    let config: AppInfo = serde_yaml::from_str("name: Infinity").unwrap();
    assert_eq!(config.name, "Infinity");
}

#[test]
fn server_config_from_yaml() {
    let config: ServerConfig = serde_yaml::from_str(
        r#"
host: "127.0.0.1"
port: 9090
"#,
    )
    .unwrap();

    assert_eq!(config.host, "127.0.0.1");
    assert_eq!(config.port, 9090);
}

#[test]
fn database_config_from_yaml() {
    let config: DatabaseConfig = serde_yaml::from_str(
        r#"
url: "postgres://localhost/infinity"
max_connections: 20
"#,
    )
    .unwrap();

    assert_eq!(config.url, "postgres://localhost/infinity");
    assert_eq!(config.max_connections, 20);
}

#[test]
fn redis_config_from_yaml() {
    let config: RedisConfig = serde_yaml::from_str(
        r#"
host: "redis.local"
port: 6380
"#,
    )
    .unwrap();

    assert_eq!(config.host, "redis.local");
    assert_eq!(config.port, 6380);
}

#[test]
fn ai_config_from_yaml() {
    let config: AiConfig = serde_yaml::from_str(
        r#"
provider: "anthropic"
api_key: "sk-ant-xxx"
"#,
    )
    .unwrap();

    assert_eq!(config.provider, "anthropic");
    assert_eq!(config.api_key.as_deref(), Some("sk-ant-xxx"));
}

#[test]
fn full_app_config_from_yaml() {
    let config: AppConfig = serde_yaml::from_str(
        r#"
app:
  name: "Infinity"
server:
  host: "0.0.0.0"
  port: 3000
database:
  url: "postgres://user:pass@localhost/infinity"
  max_connections: 10
redis:
  host: "127.0.0.1"
  port: 6379
ai:
  provider: "openai"
  api_key: "sk-test"
"#,
    )
    .unwrap();

    assert_eq!(config.app.name, "Infinity");
    assert_eq!(config.server.port, 3000);
    assert_eq!(config.database.max_connections, 10);
    assert_eq!(config.redis.host, "127.0.0.1");
    assert_eq!(config.ai.api_key.as_deref(), Some("sk-test"));
}

#[test]
fn yaml_parse_error() {
    let result = serde_yaml::from_str::<AppConfig>(
        r#"
app:
  name: "Infinity"
server:
  host: "0.0.0.0"
  port: "not-a-number"
"#,
    );

    assert!(result.is_err());
}

#[test]
fn yaml_missing_sections_use_defaults() {
    let config = serde_yaml::from_str::<AppConfig>(
        r#"
server:
  host: "0.0.0.0"
  port: 8080
"#,
    )
    .unwrap();

    assert_eq!(config.app.name, "Infinity");
    assert_eq!(config.database.max_connections, 10);
    assert_eq!(config.redis.port, 6379);
}

#[test]
fn toml_vs_yaml_consistency() {
    let from_yaml: ServerConfig = serde_yaml::from_str(
        r#"
host: "10.0.0.1"
port: 443
"#,
    )
    .unwrap();
    let from_toml: ServerConfig = toml::from_str(
        r#"
host = "10.0.0.1"
port = 443
"#,
    )
    .unwrap();

    assert_eq!(from_yaml.host, from_toml.host);
    assert_eq!(from_yaml.port, from_toml.port);
}

#[test]
fn yaml_roundtrip() {
    let config: ServerConfig = serde_yaml::from_str(
        r#"
host: "0.0.0.0"
port: 8080
"#,
    )
    .unwrap();
    let serialized = serde_yaml::to_string(&config).unwrap();
    let restored: ServerConfig = serde_yaml::from_str(&serialized).unwrap();

    assert_eq!(config.host, restored.host);
    assert_eq!(config.port, restored.port);
}

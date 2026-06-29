use infinity_config::config::{
    AppInfo,
    ai::AiConfig,
    config::AppConfig,
    database::DatabaseConfig,
    redis::RedisConfig,
    server::ServerConfig,
};

/// 测试从 YAML 反序列化 AppInfo
#[test]
fn test_app_info_from_yaml() {
    let yaml = r#"
name: Infinity
"#;
    let config: AppInfo = serde_yaml::from_str(yaml).unwrap();
    assert_eq!(config.name, "Infinity");
}

/// 测试从 YAML 反序列化 ServerConfig
#[test]
fn test_server_config_from_yaml() {
    let yaml = r#"
host: "127.0.0.1"
port: 9090
"#;
    let config: ServerConfig = serde_yaml::from_str(yaml).unwrap();
    assert_eq!(config.host, "127.0.0.1");
    assert_eq!(config.port, 9090);
}

/// 测试从 YAML 反序列化 DatabaseConfig
#[test]
fn test_database_config_from_yaml() {
    let yaml = r#"
url: "postgres://localhost/infinity"
max_connections: 20
"#;
    let config: DatabaseConfig = serde_yaml::from_str(yaml).unwrap();
    assert_eq!(config.url, "postgres://localhost/infinity");
    assert_eq!(config.max_connections, 20);
}

/// 测试从 YAML 反序列化 RedisConfig
#[test]
fn test_redis_config_from_yaml() {
    let yaml = r#"
host: "redis.local"
port: 6380
"#;
    let config: RedisConfig = serde_yaml::from_str(yaml).unwrap();
    assert_eq!(config.host, "redis.local");
    assert_eq!(config.port, 6380);
}

/// 测试从 YAML 反序列化 AiConfig
#[test]
fn test_ai_config_from_yaml() {
    let yaml = r#"
provider: "anthropic"
api_key: "sk-ant-xxx"
"#;
    let config: AiConfig = serde_yaml::from_str(yaml).unwrap();
    assert_eq!(config.provider, "anthropic");
    assert_eq!(config.api_key, "sk-ant-xxx");
}

/// 测试从 YAML 反序列化完整 AppConfig
#[test]
fn test_full_app_config_from_yaml() {
    let yaml = r#"
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
"#;
    let config: AppConfig = serde_yaml::from_str(yaml).unwrap();

    assert_eq!(config.app.name, "Infinity");
    assert_eq!(config.server.host, "0.0.0.0");
    assert_eq!(config.server.port, 3000);
    assert_eq!(config.database.url, "postgres://user:pass@localhost/infinity");
    assert_eq!(config.database.max_connections, 10);
    assert_eq!(config.redis.host, "127.0.0.1");
    assert_eq!(config.redis.port, 6379);
    assert_eq!(config.ai.provider, "openai");
    assert_eq!(config.ai.api_key, "sk-test");
}

/// 测试 YAML 解析失败时抛出错误
#[test]
fn test_yaml_parse_error() {
    let yaml = r#"
app:
  name: "Infinity"
server:
  host: "0.0.0.0"
  port: "not-a-number"
"#;
    let result = serde_yaml::from_str::<AppConfig>(yaml);
    assert!(result.is_err(), "port 类型错误应导致解析失败");
}

/// 测试 YAML 缺失必填字段时抛出错误
#[test]
fn test_yaml_missing_field_error() {
    let yaml = r#"
server:
  host: "0.0.0.0"
  port: 8080
"#;
    let result = serde_yaml::from_str::<AppConfig>(yaml);
    assert!(result.is_err(), "缺少 app 字段应导致解析失败");
}

/// 测试 TOML 与 YAML 反序列化结果一致性
#[test]
fn test_toml_vs_yaml_consistency() {
    let yaml = r#"
host: "10.0.0.1"
port: 443
"#;
    let toml_str = r#"
host = "10.0.0.1"
port = 443
"#;

    let from_yaml: ServerConfig = serde_yaml::from_str(yaml).unwrap();
    let from_toml: ServerConfig = toml::from_str(toml_str).unwrap();

    assert_eq!(from_yaml.host, from_toml.host);
    assert_eq!(from_yaml.port, from_toml.port);
}

/// 测试 YAML 反序列化后再序列化为 YAML（往返）
#[test]
fn test_yaml_roundtrip() {
    let yaml = r#"
host: "0.0.0.0"
port: 8080
"#;

    let config: ServerConfig = serde_yaml::from_str(yaml).unwrap();
    let serialized = serde_yaml::to_string(&config).unwrap();
    let restored: ServerConfig = serde_yaml::from_str(&serialized).unwrap();

    assert_eq!(config.host, restored.host);
    assert_eq!(config.port, restored.port);
}

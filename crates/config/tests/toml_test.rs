use infinity_config::{
    config::{
        AppInfo,
        config::AppConfig,
        server::ServerConfig,
    },
    error::ConfigError,
};

/// 测试从 TOML 反序列化 AppInfo
#[test]
fn test_app_info_from_toml() {
    let toml_str = r#"name = "test""#;
    let config: AppInfo = toml::from_str(toml_str).unwrap();
    assert_eq!(config.name, "test");
}

/// 测试从 TOML 反序列化 ServerConfig
#[test]
fn test_server_config_from_toml() {
    let toml_str = r#"
host = "127.0.0.1"
port = 9090
"#;
    let config: ServerConfig = toml::from_str(toml_str).unwrap();
    assert_eq!(config.host, "127.0.0.1");
    assert_eq!(config.port, 9090);
}

/// 测试从 TOML 反序列化完整 AppConfig
#[test]
fn test_full_app_config_from_toml() {
    let toml_str = r#"
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
"#;
    let config: AppConfig = toml::from_str(toml_str).unwrap();

    assert_eq!(config.app.name, "Infinity");
    assert_eq!(config.server.host, "0.0.0.0");
    assert_eq!(config.server.port, 3000);
    assert_eq!(config.database.url, "postgres://localhost/db");
    assert_eq!(config.database.max_connections, 10);
    assert_eq!(config.redis.host, "127.0.0.1");
    assert_eq!(config.redis.port, 6379);
    assert_eq!(config.ai.provider, "openai");
    assert_eq!(config.ai.api_key, "sk-test");
}

/// 测试 TOML 解析失败时抛出错误
#[test]
fn test_toml_parse_error() {
    let toml_str = r#"
[app]
name = "test"

[server]
host = "0.0.0.0"
port = "not-a-number"
"#;
    let result = toml::from_str::<AppConfig>(toml_str);
    assert!(result.is_err(), "port 类型错误应导致解析失败");
}

/// 测试 TOML 缺失必填字段时抛出错误
#[test]
fn test_toml_missing_field_error() {
    let toml_str = r#"
[server]
host = "0.0.0.0"
port = 8080
"#;
    let result = toml::from_str::<AppConfig>(toml_str);
    assert!(result.is_err(), "缺少 app 字段应导致解析失败");
}

/// 测试 TOML 反序列化后再序列化（往返）
#[test]
fn test_toml_roundtrip() {
    let toml_str = r#"
host = "0.0.0.0"
port = 8080
"#;

    let config: ServerConfig = toml::from_str(toml_str).unwrap();
    let serialized = toml::to_string(&config).unwrap();
    let restored: ServerConfig = toml::from_str(&serialized).unwrap();

    assert_eq!(config.host, restored.host);
    assert_eq!(config.port, restored.port);
}

/// 测试配置错误类型的构造与显示
#[test]
fn test_config_error_display() {
    // FileNotFound
    let err = ConfigError::file_not_found("/tmp/missing.toml");
    assert_eq!(err.to_string(), "配置文件不存在: /tmp/missing.toml");

    // MissingField
    let err = ConfigError::missing_field("app.name");
    assert_eq!(err.to_string(), "配置项缺失: app.name");
}

/// 测试 std::io::Error 到 ConfigError 的转换
#[test]
fn test_io_error_conversion() {
    // NotFound → FileNotFound
    let io_err = std::io::Error::from(std::io::ErrorKind::NotFound);
    let config_err: ConfigError = io_err.into();
    assert!(matches!(config_err, ConfigError::FileNotFound(_)));

    // 其他 IO 错误 → FileReadError
    let io_err = std::io::Error::from(std::io::ErrorKind::PermissionDenied);
    let config_err: ConfigError = io_err.into();
    assert!(matches!(config_err, ConfigError::FileReadError(_)));
}

/// 测试 serde_json::Error 到 ConfigError 的转换
#[test]
fn test_json_error_conversion() {
    let json_err = serde_json::from_str::<serde_json::Value>("invalid json").unwrap_err();
    let config_err: ConfigError = json_err.into();
    assert!(matches!(config_err, ConfigError::ParseError(_)));
}

/// 测试 serde_yaml::Error 到 ConfigError 的转换
#[test]
fn test_yaml_error_conversion() {
    let yaml_err = serde_yaml::from_str::<serde_json::Value>(": invalid").unwrap_err();
    let config_err: ConfigError = yaml_err.into();
    assert!(matches!(config_err, ConfigError::ParseError(_)));
}

/// 测试 ConfigError 的 Debug / Display / Serialize / Deserialize
#[test]
fn test_config_error_roundtrip() {
    let original = ConfigError::ValidationError("port 必须在 1..65535 之间".to_string());

    let json = serde_json::to_string(&original).expect("序列化失败");
    let restored: ConfigError = serde_json::from_str(&json).expect("反序列化失败");

    assert_eq!(original.to_string(), restored.to_string());
}

/// 测试所有错误变体均可正确构造
#[test]
fn test_all_error_variants() {
    let variants = vec![
        ConfigError::FileNotFound("a".into()),
        ConfigError::FileReadError("b".into()),
        ConfigError::ParseError("c".into()),
        ConfigError::EnvVarError("d".into()),
        ConfigError::MissingField("e".into()),
        ConfigError::InvalidType("f".into()),
        ConfigError::ValidationError("g".into()),
        ConfigError::OutOfRange("h".into()),
        ConfigError::InvalidFormat("i".into()),
        ConfigError::Unknown("j".into()),
    ];

    for v in &variants {
        let s = v.to_string();
        assert!(!s.is_empty(), "错误信息不应为空");
    }

    // 确认每个变体互不相同
    for i in 0..variants.len() {
        for j in (i + 1)..variants.len() {
            assert_ne!(
                std::mem::discriminant(&variants[i]),
                std::mem::discriminant(&variants[j]),
                "错误变体 {} 和 {} 不应相同",
                i,
                j
            );
        }
    }
}

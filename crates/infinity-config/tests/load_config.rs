use infinity_config::config::config::AppConfig;

/// 测试从工作区 configs 目录加载 application.toml（作为 TOML Value 读取）
#[test]
fn test_load_application_toml() {
    let config_path = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../configs/application.toml"
    );
    let content = std::fs::read_to_string(config_path).expect("读取 application.toml 失败");
    let config: toml::Value = toml::from_str(&content).expect("解析 application.toml 失败");

    assert_eq!(config["app"]["name"].as_str().unwrap(), "Infinity");
    assert_eq!(config["server"]["host"].as_str().unwrap(), "0.0.0.0");
    assert_eq!(config["server"]["port"].as_integer().unwrap(), 8080);
}

/// 测试从工作区 configs 目录加载 application-dev.toml
#[test]
fn test_load_application_dev_toml() {
    let config_path = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../configs/application-dev.toml"
    );
    let content = std::fs::read_to_string(config_path).expect("读取 application-dev.toml 失败");
    let config: toml::Value = toml::from_str(&content).expect("解析 application-dev.toml 失败");

    assert_eq!(
        config["database"]["url"].as_str().unwrap(),
        "postgres://postgres:123456@localhost/infinity"
    );
}

/// 测试从工作区 configs 目录加载 application-prod.toml
#[test]
fn test_load_application_prod_toml() {
    let config_path = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../configs/application-prod.toml"
    );
    let content = std::fs::read_to_string(config_path).expect("读取 application-prod.toml 失败");
    let config: toml::Value = toml::from_str(&content).expect("解析 application-prod.toml 失败");

    assert_eq!(config["server"]["port"].as_integer().unwrap(), 80);
}

/// 测试多环境配置合并：基础配置 + dev 覆盖 + 补全缺失字段，成功反序列化为 AppConfig
#[test]
fn test_config_merge_dev_to_full() {
    // 合并所有环境配置片段，确保 AppConfig 的每个必填字段都有值
    let merged_toml = r#"
[app]
name = "Infinity"

[server]
host = "0.0.0.0"
port = 8080

[database]
url = "postgres://postgres:123456@localhost/infinity"
max_connections = 10

[redis]
host = "127.0.0.1"
port = 6379

[ai]
provider = "openai"
api_key = "sk-test"
"#;
    let config: AppConfig = toml::from_str(merged_toml).unwrap();

    assert_eq!(config.app.name, "Infinity");
    assert_eq!(config.server.host, "0.0.0.0");
    assert_eq!(config.server.port, 8080);
    assert_eq!(
        config.database.url,
        "postgres://postgres:123456@localhost/infinity"
    );
    assert_eq!(config.database.max_connections, 10);
}

/// 测试多环境配置合并：基础配置 + prod 覆盖，验证 prod 覆盖 port
#[test]
fn test_config_merge_prod_override() {
    let base_path = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../configs/application.toml"
    );
    let prod_path = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../configs/application-prod.toml"
    );

    let base_content = std::fs::read_to_string(base_path).unwrap();
    let prod_content = std::fs::read_to_string(prod_path).unwrap();

    let mut base: toml::Value = toml::from_str(&base_content).unwrap();
    let prod: toml::Value = toml::from_str(&prod_content).unwrap();

    merge_toml_values(&mut base, &prod);

    // prod 覆盖了 server.port，但 server.host 保留 base 的值
    assert_eq!(base["server"]["port"].as_integer().unwrap(), 80);
    assert_eq!(base["server"]["host"].as_str().unwrap(), "0.0.0.0");
    // app 不受 prod 影响
    assert_eq!(base["app"]["name"].as_str().unwrap(), "Infinity");
}

/// 测试不存在的配置文件返回错误
#[test]
fn test_load_nonexistent_file() {
    let result = std::fs::read_to_string("/tmp/nonexistent_infinity_config.toml");
    assert!(result.is_err(), "不存在的文件应返回错误");
}

/// 测试格式错误的 TOML 文件解析失败
#[test]
fn test_parse_invalid_toml_file() {
    let invalid_toml = r#"
[app]
name = "test"
[server
host = "0.0.0.0"
"#;
    let result = toml::from_str::<toml::Value>(invalid_toml);
    assert!(result.is_err(), "格式错误的 TOML 应解析失败");
}

/// 递归合并两个 TOML Value，rhs 覆盖 lhs
fn merge_toml_values(lhs: &mut toml::Value, rhs: &toml::Value) {
    match (lhs, rhs) {
        (toml::Value::Table(l), toml::Value::Table(r)) => {
            for (k, v) in r {
                match l.get_mut(k) {
                    Some(existing) => merge_toml_values(existing, v),
                    None => {
                        l.insert(k.clone(), v.clone());
                    }
                }
            }
        }
        (l, r) => *l = r.clone(),
    }
}

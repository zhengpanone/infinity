use infinity_config::{Config, ConfigError, ConfigManager, Environment, config::AppConfig};

fn workspace_configs_dir() -> &'static str {
    concat!(env!("CARGO_MANIFEST_DIR"), "/../../apps/admin/configs")
}

#[test]
fn builder_loads_from_dir_with_env() {
    let config = Config::builder()
        .dir(workspace_configs_dir())
        .env(Environment::Prod)
        .build()
        .unwrap();

    assert_eq!(config.server.port, 80);
    assert_eq!(config.database.max_connections, 50);
}

#[test]
fn builder_accepts_explicit_config() {
    let config = AppConfig::default();
    let built = Config::builder().config(config.clone()).build().unwrap();

    assert_eq!(built.app.name, config.app.name);
    assert_eq!(built.server.port, config.server.port);
}

#[test]
fn config_entry_reports_unimplemented_operations() {
    assert!(matches!(Config::reload(), Err(ConfigError::Unsupported(_))));
    assert!(matches!(
        Config::shutdown(),
        Err(ConfigError::Unsupported(_))
    ));
}

#[test]
fn manager_and_config_builder_share_loading_semantics() {
    let from_manager =
        ConfigManager::load_from_dir_with_env(workspace_configs_dir(), Environment::Dev).unwrap();
    let from_builder = Config::builder()
        .dir(workspace_configs_dir())
        .env(Environment::Dev)
        .build()
        .unwrap();

    assert_eq!(from_manager.database.url, from_builder.database.url);
    assert_eq!(from_manager.logger.unwrap().level, "debug");
}

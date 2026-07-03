use infinity_config::{Config, config::AppConfig};

fn main() -> Result<(), infinity_config::ConfigError> {
    // 已经有类型化配置时，可以直接初始化全局单例。
    let config = AppConfig::default();
    let _ = Config::from_config(config)?;

    println!("config initialized");
    Ok(())
}

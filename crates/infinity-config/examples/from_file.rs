use infinity_config::loader;

fn main() -> Result<(), infinity_config::ConfigError> {
    // 单文件加载适合不需要分层覆盖的场景。
    let config: infinity_config::config::AppConfig = loader::load_file("configs/application.toml")?;

    println!("app = {}", config.app.name);
    Ok(())
}

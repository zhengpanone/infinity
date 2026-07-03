use infinity_config::{Config, Environment};

fn main() -> Result<(), infinity_config::ConfigError> {
    // 显式指定环境，加载对应的覆盖配置。
    let config = Config::builder()
        .dir("configs")
        .env(Environment::Prod)
        .build()?;

    println!("prod port = {}", config.server.port);
    Ok(())
}

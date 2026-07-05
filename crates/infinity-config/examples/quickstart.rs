use infinity_config::Config;

fn main() -> Result<(), infinity_config::ConfigError> {
    // 从默认 configs 目录按当前环境加载。
    let config = Config::builder().init()?;

    println!("{}:{}", config.server.host, config.server.port);
    Ok(())
}

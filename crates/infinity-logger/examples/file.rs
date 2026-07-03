//! 文件日志示例。
//!
//! 运行：`cargo run --example file --features file`

use infinity_logger::{Logger, config::Rotation};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    Logger::builder()
        .console(true)
        .file(true)
        .directory("./logs")
        .filename("example")
        .rotation(Rotation::Daily)
        .init()?;

    tracing::info!("This message goes to both console and ./logs/example.<date>");
    tracing::info!(user_id = 123, action = "login", "User activity recorded");

    Ok(())
}

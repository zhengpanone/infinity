//! JSON 结构化日志示例。
//!
//! 运行：`cargo run --example json --features json`

use infinity_logger::Logger;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    Logger::builder().console(false).json(true).init()?;

    tracing::info!(
        user_id = 123,
        method = "POST",
        path = "/api/login",
        duration_ms = 45,
        "Request completed"
    );

    Ok(())
}

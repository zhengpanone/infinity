//! 基础用法示例。
//!
//! 运行：`cargo run --example basic`

use infinity_logger::Logger;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    Logger::builder().init()?;

    tracing::info!("Hello, Infinity!");
    tracing::debug!("This debug message is filtered out by default (info level)");
    tracing::warn!(code = 42, "A warning with a structured field");

    Ok(())
}

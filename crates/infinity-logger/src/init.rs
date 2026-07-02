use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};

use crate::{LoggerError, Result, config::LoggerConfig};

/// 初始化 tracing subscriber。
///
/// 该函数属于 crate 内部实现，不对外公开。
pub(crate) fn init(config: &LoggerConfig) -> Result<()> {
    let filter = build_filter(config);

    let subscriber = tracing_subscriber::registry()
        .with(filter)
        .with(crate::layer::console::layer(config));

    subscriber
        .try_init()
        .map_err(|_| LoggerError::AlreadyInitialized)?;

    Ok(())
}

fn build_filter(config: &LoggerConfig) -> EnvFilter {
    EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(config.level.as_str()))
}

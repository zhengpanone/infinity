use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};

use crate::{LoggerError, Result, config::LoggerConfig, layer::BoxLayer};

pub(crate) fn init(config: LoggerConfig) -> Result<()> {
    let filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(config.level.as_str()));

    // 收集所有启用的 layer
    let mut layers: Vec<BoxLayer> = Vec::new();

    #[cfg(feature = "console")]
    {
        if let Some(l) = crate::layer::console::layer(&config) {
            layers.push(Box::new(l));
        }
    }

    #[cfg(feature = "file")]
    {
        if let Some(l) = crate::layer::file::layer(&config) {
            layers.push(Box::new(l));
        }
    }

    #[cfg(feature = "json")]
    {
        if let Some(l) = crate::layer::json::layer(&config) {
            layers.push(Box::new(l));
        }
    }

    #[cfg(feature = "otel")]
    {
        if let Some(l) = crate::layer::otel::layer(&config) {
            layers.push(Box::new(l));
        }
    }

    tracing_subscriber::registry()
        .with(layers)
        .with(filter)
        .try_init()
        .map_err(|_| LoggerError::AlreadyInitialized)?;

    Ok(())
}

#[cfg(any(
    feature = "console",
    feature = "file",
    feature = "json",
    feature = "otel"
))]
use tracing_subscriber::Layer;
use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};

use crate::{LoggerError, Result, config::LoggerConfig};

/// 初始化 tracing subscriber。
///
/// 该函数属于 crate 内部实现，不对外公开。
pub(crate) fn init(config: &LoggerConfig) -> Result<()> {
    let filter = build_filter(config);

    // 收集所有启用的 layer。不同 layer 是不同的具体类型，
    // 因此统一装箱为 `BoxLayer` 后放入 Vec。
    #[cfg_attr(
        not(any(
            feature = "console",
            feature = "file",
            feature = "json",
            feature = "otel"
        )),
        allow(unused_mut)
    )]
    let mut layers: Vec<crate::layer::BoxLayer> = Vec::new();

    #[cfg(feature = "console")]
    {
        if let Some(l) = crate::layer::console::layer(config) {
            layers.push(l.boxed());
        }
    }

    #[cfg(feature = "file")]
    {
        if let Some(l) = crate::layer::file::layer(config) {
            layers.push(l.boxed());
        }
    }

    #[cfg(feature = "json")]
    {
        if let Some(l) = crate::layer::json::layer(config) {
            layers.push(l.boxed());
        }
    }

    #[cfg(feature = "otel")]
    {
        if let Some(l) = crate::layer::otel::layer(config) {
            layers.push(l.boxed());
        }
    }

    tracing_subscriber::registry()
        .with(layers)
        .with(filter)
        .try_init()
        .map_err(|_| LoggerError::AlreadyInitialized)?;

    Ok(())
}

fn build_filter(config: &LoggerConfig) -> EnvFilter {
    EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(config.level.as_str()))
}

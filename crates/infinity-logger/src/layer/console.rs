use tracing_subscriber::{Layer, Registry, fmt};

use crate::config::LoggerConfig;

/// 控制台 Layer
pub fn layer(config: &LoggerConfig) -> Option<impl Layer<Registry> + Send + Sync> {
    if !config.console.enabled {
        return None;
    }
    let layer = fmt::layer()
        .with_ansi(config.console.ansi)
        .with_target(config.console.target || config.format.with_target)
        .with_file(config.console.file || config.format.with_file)
        .with_line_number(config.console.line_number || config.format.with_line_number)
        .with_thread_ids(config.console.thread_id || config.format.with_thread_id)
        .with_thread_names(config.console.thread_name || config.format.with_thread_name);
    Some(layer)
}

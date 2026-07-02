use tracing_subscriber::{Layer, Registry, fmt};

use crate::config::LoggerConfig;

/// JSON Layer
pub fn layer(config: &LoggerConfig) -> Option<impl Layer<Registry> + Send + Sync> {
    if !config.json.enabled {
        return None;
    }
    let layer = fmt::layer()
        .json()
        .with_ansi(false)
        .with_target(config.format.with_target)
        .with_file(config.format.with_file)
        .with_line_number(config.format.with_line_number)
        .with_thread_ids(config.format.with_thread_id)
        .with_thread_names(config.format.with_thread_name)
        .with_current_span(true)
        .with_span_list(false);

    Some(layer)
}

use tracing_subscriber::fmt;

use crate::config::LoggerConfig;
/// 控制台 Layer
pub fn apply<S, N>(layer: fmt::Layer<S, N>, config: &LoggerConfig) -> fmt::Layer<S, N>
where
    S: tracing::Subscriber,
    N: for<'writer> tracing_subscriber::fmt::FormatFields<'writer> + 'static,
{
    layer
        .with_ansi(config.console.ansi)
        .with_target(config.format.with_target)
        .with_file(config.format.with_file)
        .with_line_number(config.format.with_line_number)
        .with_thread_names(config.format.with_thread_name)
        .with_thread_ids(config.format.with_thread_id)
}

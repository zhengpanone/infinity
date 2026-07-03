use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::{Layer, Registry, fmt};

use crate::config::LoggerConfig;

/// 文件日志 Layer
pub fn layer(config: &LoggerConfig) -> Option<impl Layer<Registry> + Send + Sync> {
    if !config.file.enabled {
        return None;
    }

    let rotation = match config.file.rotation {
        crate::config::Rotation::Never => Rotation::NEVER,
        crate::config::Rotation::Minutely => Rotation::MINUTELY,
        crate::config::Rotation::Hourly => Rotation::HOURLY,
        crate::config::Rotation::Daily => Rotation::DAILY,
    };

    let file_appender =
        RollingFileAppender::new(rotation, &config.file.directory, &config.file.filename);

    let layer = fmt::layer()
        .with_writer(file_appender)
        .with_ansi(false)
        .with_target(config.format.with_target)
        .with_file(config.format.with_file)
        .with_line_number(config.format.with_line_number)
        .with_thread_ids(config.format.with_thread_id)
        .with_thread_names(config.format.with_thread_name);

    Some(layer)
}

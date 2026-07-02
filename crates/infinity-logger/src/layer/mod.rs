use tracing_subscriber::{Layer, Registry};

/// 统一 Layer 类型
pub type BoxLayer = Box<dyn Layer<Registry> + Send + Sync>;

pub mod console;

#[cfg(feature = "file")]
pub mod file;

#[cfg(feature = "json")]
pub mod json;

#[cfg(feature = "otel")]
pub mod otel;

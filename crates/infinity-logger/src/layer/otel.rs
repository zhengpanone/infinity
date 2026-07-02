use opentelemetry::trace::TracerProvider;
use opentelemetry_sdk::{trace::SdkTracerProvider, Resource};
use tracing_opentelemetry::OpenTelemetryLayer;
use tracing_subscriber::{Layer, Registry};

use crate::config::LoggerConfig;

/// OpenTelemetry Layer
pub fn layer(_config: &LoggerConfig) -> Option<impl Layer<Registry> + Send + Sync> {
    let tracer_provider: SdkTracerProvider = SdkTracerProvider::builder()
        .with_resource(Resource::builder_empty().build())
        .build();

    let tracer = tracer_provider.tracer("infinity");

    let layer = OpenTelemetryLayer::new(tracer);

    Some(layer)
}

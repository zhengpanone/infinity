//! Axum 集成。
//!
//! 提供基于 `tower-http` 的 HTTP 请求追踪 Layer，自动为每个请求创建 span，
//! 记录 method、uri、status、耗时等信息。

use tower_http::trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer};
use tracing::Level;

/// 创建一个用于 Axum / Tower 的请求追踪 Layer。
///
/// 该 Layer 会为每个请求创建一个 `INFO` 级别的 span，并在响应完成时记录耗时。
///
/// # Examples
///
/// ```
/// // 将该 Layer 应用到 axum Router：
/// //   Router::new().layer(infinity_logger::middleware::axum::trace_layer());
/// let _layer = infinity_logger::middleware::axum::trace_layer();
/// ```
pub fn trace_layer()
-> TraceLayer<tower_http::classify::SharedClassifier<tower_http::classify::ServerErrorsAsFailures>>
{
    TraceLayer::new_for_http()
        .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
        .on_response(DefaultOnResponse::new().level(Level::INFO))
}

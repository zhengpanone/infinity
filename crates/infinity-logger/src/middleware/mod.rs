//! 框架集成中间件。
//!
//! 负责为 HTTP / gRPC 请求注入上下文（Request ID、TraceId、Span），
//! 不影响 Logger 本身的初始化。

pub mod request_id;

#[cfg(feature = "axum")]
pub mod axum;

#[cfg(feature = "grpc")]
pub mod grpc;

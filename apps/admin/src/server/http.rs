//! Admin HTTP 服务。
//!
//! 基于 `axum` 提供最小可用的管理端 HTTP 服务：健康检查端点、基于
//! `infinity-logger` 的请求追踪，并复用 `infinity-web` 的
//! [`ApiError`](infinity_web::ApiError) 统一错误响应。优雅关闭信号来自
//! [`super::shutdown_signal`]。

use axum::{Json, Router, extract::Path, routing::get};
use serde::Serialize;

use infinity_config::config::AppConfig;
use infinity_error::{ErrorKind, InfinityError, Result, ResultExt};
use infinity_web::WebResult;

use crate::VERSION;

/// 健康检查响应体。
#[derive(Debug, Serialize)]
struct Health {
    /// 固定为 `"ok"`，探针据此判断存活。
    status: &'static str,
    /// 服务版本，便于确认部署产物。
    version: &'static str,
}

/// 构建 admin HTTP 路由。
///
/// 拆分为独立函数便于在测试中直接构造并驱动路由。
pub(crate) fn router() -> Router {
    Router::new()
        .route("/", get(root))
        .route("/health", get(health))
        .route("/admins/{id}", get(get_admin))
        .layer(infinity_logger::middleware::axum::trace_layer())
}

/// 根路径：返回服务标识。
async fn root() -> &'static str {
    "infinity-admin-server"
}

/// 健康检查：返回服务状态与版本，供负载均衡 / 探针使用。
async fn health() -> Json<Health> {
    Json(Health {
        status: "ok",
        version: VERSION,
    })
}

/// 按 ID 查询管理员（占位实现）。
///
/// 用于演示 `infinity-web` 的错误→响应映射：非法 ID 返回 `400 validation`，
/// 合法但暂无存储时返回 `404 not_found`。`InfinityError` 经 `?` 自动转换为
/// [`ApiError`](infinity_web::ApiError) 并序列化为标准错误响应。
async fn get_admin(Path(id): Path<String>) -> WebResult<Json<Health>> {
    if id.trim().is_empty() || id.contains('/') {
        return Err(InfinityError::validation_field("id", "must be a non-empty identifier").into());
    }

    // 骨架：尚未接入数据库，统一返回 not_found，演示 4xx 客户端错误映射。
    Err(InfinityError::not_found(format!("admin:{id}")).into())
}

/// 绑定配置中的 `host:port` 并启动 HTTP 服务，直到收到关闭信号后优雅退出。
pub(crate) async fn serve(config: &AppConfig) -> Result<()> {
    let addr = format!("{}:{}", config.server.host, config.server.port);
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .with_context(ErrorKind::Web, || format!("failed to bind {addr}"))?;

    tracing::info!(addr = %addr, "admin HTTP server listening");

    axum::serve(listener, router())
        .with_graceful_shutdown(super::shutdown_signal())
        .await
        .context(ErrorKind::Web, "admin HTTP server error")?;

    tracing::info!("admin HTTP server stopped");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn health_reports_ok_with_version() {
        let Json(body) = health().await;
        assert_eq!(body.status, "ok");
        assert_eq!(body.version, VERSION);
    }

    #[tokio::test]
    async fn get_admin_rejects_blank_id_as_validation_error() {
        let err = get_admin(Path("   ".to_owned())).await.unwrap_err();
        assert_eq!(err.status, 400);
        assert_eq!(err.code, "validation");
    }

    #[tokio::test]
    async fn get_admin_maps_missing_admin_to_not_found() {
        let err = get_admin(Path("42".to_owned())).await.unwrap_err();
        assert_eq!(err.status, 404);
        assert_eq!(err.code, "not_found");
    }
}

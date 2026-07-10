//! Admin HTTP 服务。
//!
//! 基于 `axum` 提供管理端 HTTP 服务：健康检查（含数据库探活）、按 ID 查询管理员，
//! 复用 `infinity-web` 的 [`ApiError`](infinity_web::ApiError) 统一错误响应，
//! 通过 [`AppState`] 注入数据库句柄。优雅关闭信号来自 [`super::shutdown_signal`]。

use axum::{
    Json, Router,
    extract::{Path, State},
    routing::get,
};
use serde::Serialize;
use std::sync::Arc;
use utoipa::OpenApi;
use utoipa_rapidoc::RapiDoc;
use utoipa_redoc::{Redoc, Servable};
use utoipa_scalar::{Scalar, Servable as ScalarServable};
use utoipa_swagger_ui::SwaggerUi;

use infinity_config::config::AppConfig;
use infinity_database::Database;
use infinity_database::repository::{AdminRecord, AdminRepository};
use infinity_error::{ErrorKind, InfinityError, Result, ResultExt};
use infinity_web::{ApiError, WebResult};

use crate::{VERSION, repository::Repositories, services::Services};
use crate::{api::http::v1_routes, handlers::ApiDoc, state::AppState};

/// 健康检查响应体。
#[derive(Debug, Serialize)]
struct Health {
    /// 固定为 `"ok"`，探针据此判断存活。
    status: &'static str,
    /// 服务版本，便于确认部署产物。
    version: &'static str,
}

/// 管理员对外视图（`AdminRecord` 的可序列化投影）。
#[derive(Debug, Serialize)]
struct AdminView {
    id: String,
    tenant_id: String,
    username: String,
}

impl From<AdminRecord> for AdminView {
    fn from(record: AdminRecord) -> Self {
        Self {
            id: record.id,
            tenant_id: record.tenant_id,
            username: record.username,
        }
    }
}

/// 构建路由器
pub fn build_router(state: AppState) -> Router {
    // 构建基础路由
    let mut router = Router::new()
        .route("/", get(root))
        .route("/health", get(health))
        .route("/get_admin", get(get_admin))
        .nest("/api/v1", v1_routes())
        .layer(infinity_logger::middleware::axum::trace_layer())
        .fallback(not_found)
        .with_state(state);
    router = add_openapi_docs(router);
    router
}

fn add_openapi_docs(router: Router) -> Router {
    // 生成 OpenAPI文档实例
    let openapi = ApiDoc::openapi();

    router
        // Swagger UI
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", openapi.clone()))
        .merge(Redoc::with_url("/redoc", openapi.clone()))
        .merge(RapiDoc::new("/api-docs/openapi.json").path("/rapidoc"))
        .merge(Scalar::with_url("/scalar", openapi))
}

async fn not_found() -> &'static str {
    "Not Found"
}

/// 根路径：返回服务标识。
async fn root() -> &'static str {
    "infinity-admin-server"
}

/// 健康检查：探活数据库后返回服务状态与版本。
///
/// 数据库不可达时返回 `503`，保持探针语义（区别于业务 5xx）。
async fn health(State(state): State<AppState>) -> WebResult<Json<Health>> {
    if let Err(err) = infinity_database::health::ping(&state.db).await {
        tracing::warn!(error = %err, "health check: database ping failed");
        // 复用 from_error 填充全部字段（对 ApiError 字段增减稳健），仅覆写探针语义所需项。
        let mut api = ApiError::from_error(&err);
        api.status = 503;
        api.message = "database unavailable".to_owned();
        return Err(api);
    }

    Ok(Json(Health {
        status: "ok",
        version: VERSION,
    }))
}

/// 按 ID 查询管理员。
///
/// 非法 ID 返回 `400 validation`；查无此人返回 `404 not_found`；命中返回 `200` 及视图。
/// `InfinityError` 经 `?` 自动转换为 [`ApiError`](infinity_web::ApiError)。
async fn get_admin(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> WebResult<Json<AdminView>> {
    let repo = AdminRepository::new(&state.db);
    match repo.find_by_id(&id).await? {
        Some(record) => Ok(Json(record.into())),
        None => Err(InfinityError::not_found(format!("admin:{id}")).into()),
    }
}

/// 绑定配置中的 `host:port` 并启动 HTTP 服务，直到收到关闭信号后优雅退出。
pub(crate) async fn serve(config: &AppConfig, db: Arc<Database>) -> Result<()> {
    let addr = format!("{}:{}", config.server.host, config.server.port);
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .with_context(ErrorKind::Web, || format!("failed to bind {addr}"))?;

    tracing::info!(addr = %addr, "admin HTTP server listening");

    let repositories = Repositories::new(db.pool().clone());
    let services = Arc::new(Services::new(repositories));
    let state = AppState { db, services };
    axum::serve(listener, build_router(state))
        .with_graceful_shutdown(super::shutdown_signal())
        .await
        .context(ErrorKind::Web, "admin HTTP server error")?;

    tracing::info!("admin HTTP server stopped");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn admin_view_maps_record_fields() {
        let record = AdminRecord {
            id: "u1".to_owned(),
            tenant_id: "t1".to_owned(),
            username: "root".to_owned(),
        };
        let view = AdminView::from(record);
        assert_eq!(view.id, "u1");
        assert_eq!(view.tenant_id, "t1");
        assert_eq!(view.username, "root");
    }
}

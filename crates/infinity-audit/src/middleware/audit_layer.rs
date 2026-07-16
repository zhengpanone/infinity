//! axum 审计中间件。
//!
//! 使用方式:
//!
//! 1. 用 [`audit_middleware`] 包装需要审计的路由(经 `axum::middleware::from_fn_with_state`);
//! 2. handler(或认证中间件)把 [`AuditContext`] 插入 response extensions,
//!    声明"本次请求是谁在对什么做什么";
//! 3. 中间件在响应产生后,结合 HTTP 状态码推断结果,组装 [`AuditLog`] 并交给
//!    [`AuditService`](crate::service::AuditService) 按策略落库。
//!
//! 未插入 [`AuditContext`] 的请求不产生审计日志,因此把中间件挂在整个
//! Router 上是安全的。

use axum::body::Body;
use axum::extract::State;
use axum::http::{Request, StatusCode, header};
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};

use crate::domain::{AuditAction, AuditLog, AuditResult, FieldChange};
use crate::service::AuditService;

/// 请求 ID 头,与 infinity-web 的 `REQUEST_ID_HEADER` 约定一致。
const REQUEST_ID_HEADER: &str = "x-request-id";

/// 一次待审计操作的上下文,由 handler 或认证中间件填写。
///
/// 通过 `response.extensions_mut().insert(ctx)` 传递给审计中间件。
#[derive(Debug, Clone)]
pub struct AuditContext {
    /// 操作者 ID。
    pub actor_id: String,
    /// 操作者显示名。
    pub actor_name: String,
    /// 操作者类型(如 `admin`)。
    pub actor_type: String,
    /// 所属租户 ID。
    pub tenant_id: String,
    /// 审计动作。
    pub action: AuditAction,
    /// 目标资源 ID。
    pub target_id: String,
    /// 目标资源显示名。
    pub target_name: String,
    /// 目标资源类型。
    pub target_type: String,
    /// 操作描述。
    pub description: String,
    /// 字段级变更(敏感字段须先经 [`mask`](crate::mask) 脱敏)。
    pub changes: Vec<FieldChange>,
}

impl AuditContext {
    /// 创建最小上下文,其余字段用 builder 风格补充。
    pub fn new(
        actor_id: impl Into<String>,
        tenant_id: impl Into<String>,
        action: AuditAction,
    ) -> Self {
        Self {
            actor_id: actor_id.into(),
            actor_name: String::new(),
            actor_type: String::new(),
            tenant_id: tenant_id.into(),
            action,
            target_id: String::new(),
            target_name: String::new(),
            target_type: String::new(),
            description: String::new(),
            changes: Vec::new(),
        }
    }

    /// 设置操作者显示名与类型。
    pub fn actor(mut self, name: impl Into<String>, actor_type: impl Into<String>) -> Self {
        self.actor_name = name.into();
        self.actor_type = actor_type.into();
        self
    }

    /// 设置目标资源。
    pub fn target(
        mut self,
        id: impl Into<String>,
        name: impl Into<String>,
        target_type: impl Into<String>,
    ) -> Self {
        self.target_id = id.into();
        self.target_name = name.into();
        self.target_type = target_type.into();
        self
    }

    /// 设置操作描述。
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = description.into();
        self
    }

    /// 追加一条字段变更。
    pub fn change(mut self, change: FieldChange) -> Self {
        self.changes.push(change);
        self
    }
}

/// 审计中间件,配合 `axum::middleware::from_fn_with_state` 使用。
///
/// ```ignore
/// let app = Router::new()
///     .route("/users/{id}", delete(delete_user))
///     .layer(middleware::from_fn_with_state(audit_service, audit_middleware));
/// ```
///
/// 从请求中采集来源信息(IP、User-Agent、请求 ID),在响应产生后读取
/// [`AuditContext`] 并落库。`MustRecord` 策略落库失败时,原响应被替换为
/// `500`,确保"审计不掉线"的合规语义。
pub async fn audit_middleware(
    State(service): State<AuditService>,
    request: Request<Body>,
    next: Next,
) -> Response {
    let ip = client_ip(&request);
    let user_agent = header_string(&request, header::USER_AGENT.as_str());
    let request_id = header_string(&request, REQUEST_ID_HEADER);

    let response = next.run(request).await;

    let Some(ctx) = response.extensions().get::<AuditContext>().cloned() else {
        return response;
    };

    let result = AuditResult::from_status_code(response.status().as_u16());
    let mut builder = AuditLog::builder(&ctx.actor_id, &ctx.tenant_id, ctx.action, result)
        .actor(&ctx.actor_name, &ctx.actor_type)
        .target(&ctx.target_id, &ctx.target_name, &ctx.target_type)
        .description(&ctx.description)
        .source(ip, user_agent, request_id);
    for change in ctx.changes {
        builder = builder.change(change);
    }

    if let Err(err) = service.record(builder.build()).await {
        tracing::error!(error = %err, "must-record audit write failed, failing request");
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            "audit log write failed",
        )
            .into_response();
    }

    response
}

/// 提取客户端 IP:优先 `X-Forwarded-For` 首个地址,其次 `X-Real-IP`。
fn client_ip(request: &Request<Body>) -> String {
    if let Some(forwarded) = request
        .headers()
        .get("x-forwarded-for")
        .and_then(|v| v.to_str().ok())
        && let Some(first) = forwarded.split(',').next()
    {
        let first = first.trim();
        if !first.is_empty() {
            return first.to_owned();
        }
    }
    header_string(request, "x-real-ip")
}

fn header_string(request: &Request<Body>, name: &str) -> String {
    request
        .headers()
        .get(name)
        .and_then(|v| v.to_str().ok())
        .unwrap_or_default()
        .to_owned()
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use axum::Router;
    use axum::middleware::from_fn_with_state;
    use axum::routing::get;
    use tower::ServiceExt;

    use super::*;
    use crate::domain::AuditQuery;
    use crate::repository::InMemoryAuditRepository;

    fn app(service: AuditService) -> Router {
        async fn audited() -> Response {
            let ctx = AuditContext::new("u-1", "t-1", AuditAction::ExportData)
                .actor("Alice", "admin")
                .target("report-1", "月度报表", "report")
                .description("导出月度报表");
            let mut response = "ok".into_response();
            response.extensions_mut().insert(ctx);
            response
        }

        async fn plain() -> &'static str {
            "no audit"
        }

        Router::new()
            .route("/audited", get(audited))
            .route("/plain", get(plain))
            .layer(from_fn_with_state(service, audit_middleware))
    }

    #[tokio::test]
    async fn records_audit_log_with_request_source() {
        let repo = Arc::new(InMemoryAuditRepository::new());
        let service = AuditService::new(repo);
        let router = app(service.clone());

        let request = Request::builder()
            .uri("/audited")
            .header("x-forwarded-for", "1.2.3.4, 10.0.0.1")
            .header(header::USER_AGENT, "curl/8")
            .header(REQUEST_ID_HEADER, "req-42")
            .body(Body::empty())
            .unwrap();
        let response = router.oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);

        let logs = service.query(&AuditQuery::default()).await.unwrap();
        assert_eq!(logs.len(), 1);
        let log = &logs[0];
        assert_eq!(log.action, AuditAction::ExportData);
        assert_eq!(log.result, AuditResult::Succeeded);
        assert_eq!(log.ip, "1.2.3.4");
        assert_eq!(log.user_agent, "curl/8");
        assert_eq!(log.request_id, "req-42");
        assert_eq!(log.target_id, "report-1");
    }

    #[tokio::test]
    async fn requests_without_context_are_not_audited() {
        let repo = Arc::new(InMemoryAuditRepository::new());
        let service = AuditService::new(repo);
        let router = app(service.clone());

        let request = Request::builder()
            .uri("/plain")
            .body(Body::empty())
            .unwrap();
        let response = router.oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);

        let logs = service.query(&AuditQuery::default()).await.unwrap();
        assert!(logs.is_empty());
    }
}

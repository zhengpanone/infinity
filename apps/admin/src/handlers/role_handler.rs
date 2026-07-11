use axum::{Json, extract::State};
use infinity_web::{ApiResponse, WebResult};
use utoipa::OpenApi;
use validator::Validate;

use crate::{
    domain::{dto::role::CreateRoleDTO, vo::role::RoleVO},
    state::AppState,
};

const TAG_NAME: &str = "Role API";

/// 创建用户
#[utoipa::path(
    post,
    path = "/create",
    tag = TAG_NAME,
    request_body(
        content = CreateRoleDTO,
        description = "创建角色参数",
        content_type = "application/json"
    ),
    responses(
        (status = 200, description = "创建成功", body = ApiResponse<RoleVO>),
        (status = 400, description = "请求参数错误"),
        (status = 401, description = "未授权"),
        (status = 403, description = "权限不足"),
        (status = 409, description = "用户已存在")
    ),
    security(
        ("jwt" = [])
    )
)]
pub async fn create(
    State(state): State<AppState>,
    Json(request): Json<CreateRoleDTO>,
) -> WebResult<ApiResponse<RoleVO>> {
    request.validate()?;
    let role = state.services.role_service.create(request).await?;
    Ok(ApiResponse::success(role))
}

/// 管理角色相关的 API 文档
#[derive(OpenApi)]
#[openapi(paths(create), components(schemas(RoleVO)),
tags((name=TAG_NAME, description="Role management"))
)]
pub struct RoleApiDoc;

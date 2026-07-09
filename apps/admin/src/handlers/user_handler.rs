use axum::extract::State;
use infinity_web::{ApiResponse, WebResult};
use tracing::debug;
use utoipa::OpenApi;

use crate::{domain::vo::user::UserVO, state::AppState};

const TAG_NAME: &str = "User API";

/// 创建用户
#[utoipa::path(
    post,
    path = "",
    tag = TAG_NAME,
    responses(
        (status = 200, description = "创建成功", body = UserVO),
        (status = 400, description = "请求参数错误"),
        (status = 401, description = "未授权"),
        (status = 403, description = "权限不足"),
        (status = 409, description = "用户已存在")
    ),
    security(
        ("jwt" = [])
    )
)]
// // #[instrument(name="http_create_user", skip_all, fields(user_id=%auth_user.user_id))]
pub async fn create_user(State(_state): State<AppState>) -> WebResult<ApiResponse<UserVO>> {
    // 检查权限
    // require_role(&auth_user, "admin")?;
    debug!("Create user:");

    Ok(ApiResponse::success_empty("创建成功"))
}

/// 管理员用户相关的 API 文档
#[derive(OpenApi)]
#[openapi(
    paths(create_user),
    components(schemas(UserVO)),
    tags((name = "Admin User API", description = "Admin User management")),
)]
pub struct UserApiDoc;

use axum::{
    Json,
    extract::{Path, State},
};
use infinity_error::InfinityError;
use infinity_web::{ApiResponse, CommonIdDTO, PaginationParams, WebResult};
use tracing::debug;
use utoipa::OpenApi;
use uuid::Uuid;
use validator::Validate;

use crate::{
    domain::{
        dto::user::{
            CheckUserExistsDTO, CreateUserDTO, UpdateUserDTO, UserQueryDTO, UserSortField,
        },
        types::ids::UserId,
        vo::user::{UserExistsVO, UserVO},
    },
    state::AppState,
};

const TAG_NAME: &str = "User API";

/// 创建用户
#[utoipa::path(
    post,
    path = "/create",
    tag = TAG_NAME,
    request_body(
        content = CreateUserDTO,
        description = "创建用户参数",
        content_type = "application/json"
    ),
    responses(
        (status = 200, description = "创建成功", body = ApiResponse<UserVO>),
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
pub async fn create(
    State(state): State<AppState>,
    Json(request): Json<CreateUserDTO>,
) -> WebResult<ApiResponse<UserVO>> {
    // 检查权限
    // require_role(&auth_user, "admin")?;
    debug!("Create user:");
    // 先做字段级校验（长度、邮箱、URL 等），失败返回 400 及具体字段错误。
    request.validate()?;
    let user = state.services.user_service.create(request).await?;
    Ok(ApiResponse::success(user))
}

/// 分页查询用户
#[utoipa::path(
    post,
    path = "/page",
    tag = TAG_NAME,
    request_body(
        content = PaginationParams<UserQueryDTO, UserSortField>,
        description = "用户分页、筛选及多字段排序参数",
        content_type = "application/json"
    ),
    responses(
        (status = 200, description = "查询成功", body = ApiResponse<Vec<UserVO>>),
        (status = 400, description = "分页、筛选或排序参数错误"),
        (status = 401, description = "未认证"),
        (status = 403, description = "权限不足"),
        (status = 500, description = "服务器或数据库错误")
    ),
    security(
        ("jwt" = [])
    )
)]
pub async fn page_list(
    State(state): State<AppState>,
    Json(request): Json<PaginationParams<UserQueryDTO, UserSortField>>,
) -> WebResult<ApiResponse<Vec<UserVO>>> {
    request.validate().map_err(InfinityError::validation)?;
    let pagination = state.services.user_service.page_list(request).await?;
    Ok(ApiResponse::paginated(pagination))
}

/// 查询用户详情
#[utoipa::path(
    get,
    path = "/detail/{id}",
    tag = TAG_NAME,
    params(
        ("id" = Uuid, Path, description = "用户 ID")
    ),
    responses(
        (status = 200, description = "查询成功", body = ApiResponse<UserVO>),
        (status = 400, description = "用户 ID 格式错误"),
        (status = 401, description = "未认证"),
        (status = 403, description = "权限不足"),
        (status = 404, description = "用户不存在"),
        (status = 500, description = "服务器或数据库错误")
    ),
    security(
        ("jwt" = [])
    )
)]
pub async fn detail(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> WebResult<ApiResponse<UserVO>> {
    let user = state
        .services
        .user_service
        .get_by_id(UserId::from(id))
        .await?;
    Ok(ApiResponse::success(user))
}

/// 更新用户
#[utoipa::path(
    post,
    path = "/update",
    tag = TAG_NAME,
    request_body(
        content = UpdateUserDTO,
        description = "用户更新参数",
        content_type = "application/json"
    ),
    responses(
        (status = 200, description = "更新成功", body = ApiResponse<UserVO>),
        (status = 400, description = "请求参数错误"),
        (status = 401, description = "未授权"),
        (status = 403, description = "权限不足"),
        (status = 404, description = "用户不存在"),
        (status = 500, description = "服务器或数据库错误")
    ),
    security(
        ("jwt" = [])
    )
)]
pub async fn update(
    State(state): State<AppState>,
    Json(request): Json<UpdateUserDTO>,
) -> WebResult<ApiResponse<UserVO>> {
    request.validate()?;
    let user = state.services.user_service.update(request).await?;
    Ok(ApiResponse::success(user))
}

/// 删除用户
#[utoipa::path(
    delete,
    path = "/delete",
    tag = TAG_NAME,
    request_body(
        content = CommonIdDTO,
        description = "待删除的用户 ID 列表",
        content_type = "application/json"
    ),
    responses(
        (status = 204, description = "删除成功"),
        (status = 400, description = "请求参数错误"),
        (status = 401, description = "未授权"),
        (status = 403, description = "权限不足"),
        (status = 500, description = "服务器或数据库错误")
    ),
    security(
        ("jwt" = [])
    )
)]
pub async fn delete(
    State(state): State<AppState>,
    Json(request): Json<CommonIdDTO>,
) -> WebResult<ApiResponse<()>> {
    request.validate()?;

    let ids = request
        .ids
        .into_iter()
        .map(UserId::from)
        .collect::<Vec<_>>();

    state.services.user_service.delete(ids).await?;
    Ok(ApiResponse::success_empty("删除成功"))
}

/// 校验邮箱或者用户名是否存在
#[utoipa::path(
    post,
    path = "/exists",
    tag = TAG_NAME,
    request_body(
        content = CheckUserExistsDTO,
        description = "校验用户名、邮箱、手机号是否已存在",
        content_type = "application/json"
    ),
    responses(
        (status = 204, description = "查询成功", body = ApiResponse<UserExistsVO>),
        (status = 400, description = "请求参数错误"),
        (status = 401, description = "未授权"),
        (status = 403, description = "权限不足"),
        (status = 500, description = "服务器或数据库错误")
    ),
    security(
        ("jwt" = [])
    )
)]
pub async fn exists(
    State(state): State<AppState>,
    Json(request): Json<CheckUserExistsDTO>,
) -> WebResult<ApiResponse<UserExistsVO>> {
    request.validate()?;

    let result = state.services.user_service.check_exists(request).await?;

    Ok(ApiResponse::success(result))
}

/// 管理员用户相关的 API 文档
#[derive(OpenApi)]
#[openapi(
    paths(create, page_list, detail, update, delete,exists),
    components(schemas(UserVO,UserExistsVO)),
    tags((name = TAG_NAME, description = "User management")),
)]
pub struct UserApiDoc;

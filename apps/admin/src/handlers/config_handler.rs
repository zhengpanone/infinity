use axum::{
    extract::{Path, State},
    Json,
};
use infinity_web::{ApiResponse, CommonIdDTO, PaginationParams, WebResult};
use utoipa::OpenApi;
use uuid::Uuid;

use crate::{
    domain::{
        dto::{
            config::{ConfigQueryDTO, ConfigSortField, CreateConfigDTO},
            user::{CheckUserExistsDTO, UpdateUserDTO},
        },
        vo::{
            config::ConfigVO,
            user::{UserExistsVO, UserVO},
        },
    },
    state::AppState,
};

const TAG_NAME: &str = "Config API";

/// 创建配置
#[utoipa::path(
    post,
    path = "/create",
    tag = TAG_NAME,
    request_body(
        content = CreateConfigDTO,
        description = "创建配置参数",
        content_type = "application/json"
    ),
    responses(
        (status = 200, description = "创建成功", body = ApiResponse),
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
    Json(request): Json<CreateConfigDTO>,
) -> WebResult<ApiResponse<()>> {
    Ok(ApiResponse::success(()))
}

/// 分页查询用户
#[utoipa::path(
    post,
    path = "/page",
    tag = TAG_NAME,
    request_body(
        content = PaginationParams<ConfigQueryDTO, ConfigSortField>,
        description = "用户分页、筛选及多字段排序参数",
        content_type = "application/json"
    ),
    responses(
        (status = 200, description = "查询成功", body = ApiResponse<Vec<ConfigVO>>),
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
    Json(request): Json<PaginationParams<ConfigQueryDTO, ConfigSortField>>,
) -> WebResult<ApiResponse<Vec<ConfigVO>>> {
    // Ok(ApiResponse::paginated(None))
    todo!()
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
) -> WebResult<ApiResponse<()>> {
    Ok(ApiResponse::success(()))
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
    Json(request): Json<()>,
) -> WebResult<ApiResponse<()>> {
    Ok(ApiResponse::success(()))
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
pub async fn exists(State(state): State<AppState>) -> WebResult<ApiResponse<()>> {
    Ok(ApiResponse::success(()))
}

/// 配置相关的 API 文档
#[derive(OpenApi)]
#[openapi(
    paths(create, page_list, detail, update, delete,exists),
    components(schemas()),
    tags((name = TAG_NAME, description = "Config management")),
)]
pub struct ConfigApiDoc;

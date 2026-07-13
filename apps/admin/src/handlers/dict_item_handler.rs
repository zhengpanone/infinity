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

use crate::domain::dto::dict_item::{
    CheckDictItemExistsDTO, CreateDictItemDTO, DictItemQueryDTO, DictItemSortField,
    UpdateDictItemDTO,
};
use crate::domain::types::ids::{ConfigCategoryId, DictItemId};
use crate::domain::vo::dict_item::{SysDictItemExistsVO, SysDictItemVO};
use crate::{
    domain::{
        dto::config_category::{
            CheckConfigCategoryExistsDTO, ConfigCategoryQueryDTO, ConfigCategorySortField,
            CreateConfigCategoryDTO, UpdateConfigCategoryDTO,
        },
        vo::config_category::{ConfigCategoryExistsVO, ConfigCategoryVO},
    },
    state::AppState,
};

const TAG_NAME: &str = "DictItem API";

/// 创建字典项
#[utoipa::path(
    post,
    path = "/create",
    tag = TAG_NAME,
    request_body(
        content = CreateConfigCategoryDTO,
        description = "创建字典项参数",
        content_type = "application/json"
    ),
    responses(
        (status = 200, description = "创建成功", body = ApiResponse<ConfigCategoryVO>),
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
    Json(request): Json<CreateDictItemDTO>,
) -> WebResult<ApiResponse<SysDictItemVO>> {
    debug!("Create Config Category:");
    request.validate()?;
    let user = state.services.dict_item_service.create(request).await?;
    Ok(ApiResponse::success(user))
}

/// 分页查询系统配置-分类
#[utoipa::path(
    post,
    path = "/page",
    tag = TAG_NAME,
    request_body(
        content = PaginationParams<ConfigCategoryQueryDTO, ConfigCategorySortField>,
        description = "系统配置分类分页、筛选及多字段排序参数",
        content_type = "application/json"
    ),
    responses(
        (status = 200, description = "查询成功", body = ApiResponse<Vec<ConfigCategoryVO>>),
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
    Json(request): Json<PaginationParams<DictItemQueryDTO, DictItemSortField>>,
) -> WebResult<ApiResponse<Vec<SysDictItemVO>>> {
    request.validate().map_err(InfinityError::validation)?;
    let pagination = state.services.dict_item_service.page_list(request).await?;
    Ok(ApiResponse::paginated(pagination))
}

/// 查询系统配置-分类详情
#[utoipa::path(
    get,
    path = "/detail/{id}",
    tag = TAG_NAME,
    params(
        ("id" = Uuid, Path, description = "系统配置-分类 ID")
    ),
    responses(
        (status = 200, description = "查询成功", body = ApiResponse<ConfigCategoryVO>),
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
) -> WebResult<ApiResponse<SysDictItemVO>> {
    let config_category = state
        .services
        .dict_item_service
        .get_by_id(DictItemId::from(id))
        .await?;
    Ok(ApiResponse::success(config_category))
}

/// 更新系统配置-分类
#[utoipa::path(
    post,
    path = "/update",
    tag = TAG_NAME,
    request_body(
        content = UpdateConfigCategoryDTO,
        description = "系统配置-分类更新参数",
        content_type = "application/json"
    ),
    responses(
        (status = 200, description = "更新成功", body = ApiResponse<ConfigCategoryVO>),
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
    Json(request): Json<UpdateDictItemDTO>,
) -> WebResult<ApiResponse<SysDictItemVO>> {
    request.validate()?;
    let config_category = state.services.dict_item_service.update(request).await?;
    Ok(ApiResponse::success(config_category))
}

/// 删除系统配置-分类
#[utoipa::path(
    delete,
    path = "/delete",
    tag = TAG_NAME,
    request_body(
        content = CommonIdDTO,
        description = "待删除的系统配置-分类 ID 列表",
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
        .map(DictItemId::from)
        .collect::<Vec<_>>();

    state.services.dict_item_service.delete(ids).await?;
    Ok(ApiResponse::success_empty("删除成功"))
}

/// 校验分类编码是否存在
#[utoipa::path(
    post,
    path = "/exists",
    tag = TAG_NAME,
    request_body(
        content = CheckConfigCategoryExistsDTO,
        description = "校验分类编码是否已存在",
        content_type = "application/json"
    ),
    responses(
        (status = 204, description = "查询成功", body = ApiResponse<ConfigCategoryExistsVO>),
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
    Json(request): Json<CheckDictItemExistsDTO>,
) -> WebResult<ApiResponse<SysDictItemExistsVO>> {
    request.validate()?;

    let result = state
        .services
        .dict_item_service
        .check_exists(request)
        .await?;

    Ok(ApiResponse::success(result))
}

/// 字典项的 API 文档
#[derive(OpenApi)]
#[openapi(
    paths(create, page_list, detail, update, delete, exists),
    components(schemas(ConfigCategoryVO, ConfigCategoryExistsVO)),
    tags((name = TAG_NAME, description = "Config Category management")),
)]
pub struct DictItemApiDoc;

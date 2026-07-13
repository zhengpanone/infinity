use utoipa::OpenApi;

pub mod config_category_handler;
pub mod config_group_handler;
pub mod config_handler;
pub mod dict_item_handler;
pub mod dict_type_handler;
pub mod role_handler;
pub mod user_handler;

/// 主 API 文档
#[derive(OpenApi)]
#[openapi(
    // 使用 info 配置 API 基本信息
    info(
        title = "系统管理文档",
        version = "1.0.0",
        description = "系统管理 API 文档",
        contact(
            name = "开发团队",
            email = "dev@example.com"
        ),
        license(name = "MIT")
    ),
    // 使用 nest 嵌套子 API
    nest(
        (path = "/user", api = user_handler::UserApiDoc),
        (path = "/role", api = role_handler::RoleApiDoc),
        (path = "/config_category", api = config_category_handler::ConfigCategoryApiDoc),
        (path = "/config_group", api = config_group_handler::ConfigGroupApiDoc),
        (path = "/config", api = config_handler::ConfigApiDoc),
        (path = "/dict_type", api = dict_type_handler::DictTypeApiDoc),
        (path = "/dict_item", api = dict_item_handler::DictItemApiDoc),
    ),
    // 服务器配置
    servers(
        (url = "/api/v1", description = "系统管理服务器"),
    ),
    // 全局安全配置
    security(
        ("jwt" = [])
    )
)]
pub struct ApiDoc;

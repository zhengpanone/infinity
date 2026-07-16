//! 单元测试：ConfigCategoryServiceImpl
//!
//! 通过 mock ConfigCategoryRepository trait 对 service 层的所有方法
//! 进行白盒测试，覆盖正常路径、错误路径、边界条件。
//!
//! 无需数据库连接，纯内存测试。

use std::sync::Arc;

use admin::domain::dto::config_category::{
    CheckConfigCategoryExistsDTO, ConfigCategoryQueryDTO, ConfigCategorySortField,
    CreateConfigCategoryDTO, UpdateConfigCategoryDTO,
};
use admin::domain::types::ids::ConfigCategoryId;
use admin::repository::config_category_repository::{
    CheckConfigCategoryExists, ConfigCategoryExistsResult, ConfigCategoryRepository,
    NewConfigCategory, UpdateConfigCategory,
};
use admin::services::config_category_service::ConfigCategoryService;
use admin::services::impls::config_category_service_impl::ConfigCategoryServiceImpl;
use chrono::Utc;
use infinity_error::{InfinityError, Result as InfResult};
use infinity_web::{PaginatedData, PaginationParams, SortOrder};
use tokio::sync::Mutex;
use uuid::Uuid;

// ---------------------------------------------------------------------------
// Mock Repository
// ---------------------------------------------------------------------------

/// Mock 仓库的行为控制：返回值 / 返回错误 / 返回 None。
enum MockBehavior<T> {
    Ok(T),
    Err(InfinityError),
}

/// 线程安全的 mock repository，外部通过 `set_xxx_behavior` 控制每个方法的返回值。
struct MockConfigCategoryRepository {
    create_behavior: Mutex<Option<MockBehavior<admin::models::config_category::ConfigCategory>>>,
    update_behavior:
        Mutex<Option<MockBehavior<Option<admin::models::config_category::ConfigCategory>>>>,
    soft_delete_behavior: Mutex<Option<MockBehavior<u64>>>,
    find_by_id_behavior:
        Mutex<Option<MockBehavior<Option<admin::models::config_category::ConfigCategory>>>>,
    page_list_behavior: Mutex<
        Option<MockBehavior<PaginatedData<Vec<admin::models::config_category::ConfigCategory>>>>,
    >,
    check_exists_behavior: Mutex<Option<MockBehavior<ConfigCategoryExistsResult>>>,
}

impl MockConfigCategoryRepository {
    fn new() -> Self {
        Self {
            create_behavior: Mutex::new(None),
            update_behavior: Mutex::new(None),
            soft_delete_behavior: Mutex::new(None),
            find_by_id_behavior: Mutex::new(None),
            page_list_behavior: Mutex::new(None),
            check_exists_behavior: Mutex::new(None),
        }
    }

    async fn set_create(&self, b: MockBehavior<admin::models::config_category::ConfigCategory>) {
        *self.create_behavior.lock().await = Some(b);
    }

    async fn set_update(
        &self,
        b: MockBehavior<Option<admin::models::config_category::ConfigCategory>>,
    ) {
        *self.update_behavior.lock().await = Some(b);
    }

    async fn set_soft_delete(&self, b: MockBehavior<u64>) {
        *self.soft_delete_behavior.lock().await = Some(b);
    }

    async fn set_find_by_id(
        &self,
        b: MockBehavior<Option<admin::models::config_category::ConfigCategory>>,
    ) {
        *self.find_by_id_behavior.lock().await = Some(b);
    }

    async fn set_page_list(
        &self,
        b: MockBehavior<PaginatedData<Vec<admin::models::config_category::ConfigCategory>>>,
    ) {
        *self.page_list_behavior.lock().await = Some(b);
    }

    async fn set_check_exists(&self, b: MockBehavior<ConfigCategoryExistsResult>) {
        *self.check_exists_behavior.lock().await = Some(b);
    }
}

#[async_trait::async_trait]
impl ConfigCategoryRepository for MockConfigCategoryRepository {
    async fn create(
        &self,
        _param: NewConfigCategory,
    ) -> InfResult<admin::models::config_category::ConfigCategory> {
        match self.create_behavior.lock().await.take().unwrap() {
            MockBehavior::Ok(v) => Ok(v),
            MockBehavior::Err(e) => Err(e),
        }
    }

    async fn update_by_id(
        &self,
        _param: UpdateConfigCategory,
    ) -> InfResult<Option<admin::models::config_category::ConfigCategory>> {
        match self.update_behavior.lock().await.take().unwrap() {
            MockBehavior::Ok(v) => Ok(v),
            MockBehavior::Err(e) => Err(e),
        }
    }

    async fn soft_delete(&self, _ids: &[ConfigCategoryId]) -> InfResult<u64> {
        match self.soft_delete_behavior.lock().await.take().unwrap() {
            MockBehavior::Ok(v) => Ok(v),
            MockBehavior::Err(e) => Err(e),
        }
    }

    async fn find_by_id(
        &self,
        _id: &ConfigCategoryId,
    ) -> InfResult<Option<admin::models::config_category::ConfigCategory>> {
        match self.find_by_id_behavior.lock().await.take().unwrap() {
            MockBehavior::Ok(v) => Ok(v),
            MockBehavior::Err(e) => Err(e),
        }
    }

    async fn page_list(
        &self,
        _query: PaginationParams<ConfigCategoryQueryDTO, ConfigCategorySortField>,
    ) -> InfResult<PaginatedData<Vec<admin::models::config_category::ConfigCategory>>> {
        match self.page_list_behavior.lock().await.take().unwrap() {
            MockBehavior::Ok(v) => Ok(v),
            MockBehavior::Err(e) => Err(e),
        }
    }

    async fn check_exists(
        &self,
        _query: &CheckConfigCategoryExists,
    ) -> InfResult<ConfigCategoryExistsResult> {
        match self.check_exists_behavior.lock().await.take().unwrap() {
            MockBehavior::Ok(v) => Ok(v),
            MockBehavior::Err(e) => Err(e),
        }
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn make_service(
    repo: Arc<MockConfigCategoryRepository>,
) -> ConfigCategoryServiceImpl {
    ConfigCategoryServiceImpl::new(repo as Arc<dyn ConfigCategoryRepository + Send + Sync>)
}

fn make_model() -> admin::models::config_category::ConfigCategory {
    let now = Utc::now();
    admin::models::config_category::ConfigCategory {
        id: ConfigCategoryId::generate(),
        category_code: "test_code".into(),
        category_name: "测试分类".into(),
        icon: Some("icon.svg".into()),
        color: Some("#ff0000".into()),
        order_num: 1,
        remark: Some("备注".into()),
        category_desc: Some("描述".into()),
        is_builtin: false,
        created_id: "user-1".into(),
        created_at: now,
        created_by: "admin".into(),
        updated_id: "user-1".into(),
        updated_at: now,
        updated_by: "admin".into(),
        is_deleted: false,
        deleted_at: None,
    }
}

fn make_create_dto() -> CreateConfigCategoryDTO {
    CreateConfigCategoryDTO {
        category_code: "new_code".into(),
        category_name: "新分类".into(),
        icon: Some("icon.png".into()),
        color: Some("#333".into()),
        order_num: 0,
        remark: Some("备注".into()),
        category_desc: Some("描述".into()),
        is_builtin: false,
    }
}

fn make_empty_paginated() -> PaginatedData<Vec<admin::models::config_category::ConfigCategory>> {
    PaginatedData::try_new(Vec::new(), 1, 20, 0).unwrap()
}

// =========================================================================
// Tests: create
// =========================================================================

#[tokio::test]
async fn create_success_returns_vo() {
    let repo = Arc::new(MockConfigCategoryRepository::new());
    let model = make_model();
    repo.set_create(MockBehavior::Ok(model.clone())).await;
    let svc = make_service(repo.clone());

    let vo = svc.create(make_create_dto()).await.unwrap();

    assert_eq!(vo.id, model.id.as_uuid());
    assert_eq!(vo.category_code, model.category_code);
    assert_eq!(vo.category_name, model.category_name);
    assert_eq!(vo.icon.as_deref(), model.icon.as_deref());
    assert_eq!(vo.color.as_deref(), model.color.as_deref());
    assert_eq!(vo.order_num, model.order_num);
}

#[tokio::test]
async fn create_repo_error_propagates() {
    let repo = Arc::new(MockConfigCategoryRepository::new());
    repo.set_create(MockBehavior::Err(InfinityError::database("连接失败")))
        .await;
    let svc = make_service(repo.clone());

    let err = svc.create(make_create_dto()).await.unwrap_err();
    assert_eq!(err.kind(), infinity_error::ErrorKind::Database);
    assert!(err.to_string().contains("连接失败"));
}

// =========================================================================
// Tests: delete
// =========================================================================

#[tokio::test]
async fn delete_empty_ids_returns_validation_error() {
    let repo = Arc::new(MockConfigCategoryRepository::new());
    let svc = make_service(repo.clone());

    let err = svc.delete(vec![]).await.unwrap_err();
    assert_eq!(err.kind(), infinity_error::ErrorKind::Validation);
    assert!(err.to_string().contains("ids is empty"));
}

#[tokio::test]
async fn delete_success_with_affected_rows() {
    let repo = Arc::new(MockConfigCategoryRepository::new());
    repo.set_soft_delete(MockBehavior::Ok(2)).await;
    let svc = make_service(repo.clone());

    let ids = vec![ConfigCategoryId::generate(), ConfigCategoryId::generate()];
    let result = svc.delete(ids).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn delete_affected_zero_returns_not_found() {
    let repo = Arc::new(MockConfigCategoryRepository::new());
    repo.set_soft_delete(MockBehavior::Ok(0)).await;
    let svc = make_service(repo.clone());

    let ids = vec![ConfigCategoryId::generate()];
    let err = svc.delete(ids).await.unwrap_err();
    assert_eq!(err.kind(), infinity_error::ErrorKind::NotFound);
}

#[tokio::test]
async fn delete_repo_error_propagates() {
    let repo = Arc::new(MockConfigCategoryRepository::new());
    repo.set_soft_delete(MockBehavior::Err(InfinityError::database("db error")))
        .await;
    let svc = make_service(repo.clone());

    let ids = vec![ConfigCategoryId::generate()];
    let err = svc.delete(ids).await.unwrap_err();
    assert_eq!(err.kind(), infinity_error::ErrorKind::Database);
}

// =========================================================================
// Tests: update
// =========================================================================

#[tokio::test]
async fn update_success_returns_vo() {
    let repo = Arc::new(MockConfigCategoryRepository::new());
    let model = make_model();
    repo.set_find_by_id(MockBehavior::Ok(Some(model.clone())))
        .await;
    repo.set_update(MockBehavior::Ok(Some(model.clone()))).await;
    let svc = make_service(repo.clone());

    let dto = UpdateConfigCategoryDTO {
        id: model.id.as_uuid(),
        category_code: Some("updated_code".into()),
        category_name: Some("更新名称".into()),
        icon: None,
        color: None,
        order_num: Some(99),
        remark: None,
        category_desc: None,
        is_builtin: None,
    };

    let vo = svc.update(dto).await.unwrap();
    assert_eq!(vo.id, model.id.as_uuid());
}

#[tokio::test]
async fn update_find_by_id_not_found_returns_error() {
    let repo = Arc::new(MockConfigCategoryRepository::new());
    repo.set_find_by_id(MockBehavior::Ok(None)).await;
    let svc = make_service(repo.clone());

    let dto = UpdateConfigCategoryDTO {
        id: Uuid::new_v4(),
        category_code: None,
        category_name: None,
        icon: None,
        color: None,
        order_num: None,
        remark: None,
        category_desc: None,
        is_builtin: None,
    };

    let err = svc.update(dto).await.unwrap_err();
    assert_eq!(err.kind(), infinity_error::ErrorKind::NotFound);
}

#[tokio::test]
async fn update_by_id_returns_none_returns_error() {
    let repo = Arc::new(MockConfigCategoryRepository::new());
    let model = make_model();
    repo.set_find_by_id(MockBehavior::Ok(Some(model.clone())))
        .await;
    repo.set_update(MockBehavior::Ok(None)).await;
    let svc = make_service(repo.clone());

    let dto = UpdateConfigCategoryDTO {
        id: model.id.as_uuid(),
        category_code: None,
        category_name: None,
        icon: None,
        color: None,
        order_num: None,
        remark: None,
        category_desc: None,
        is_builtin: None,
    };

    let err = svc.update(dto).await.unwrap_err();
    assert_eq!(err.kind(), infinity_error::ErrorKind::NotFound);
}

#[tokio::test]
async fn update_repo_error_propagates() {
    let repo = Arc::new(MockConfigCategoryRepository::new());
    repo.set_find_by_id(MockBehavior::Err(InfinityError::database("连接断开")))
        .await;
    let svc = make_service(repo.clone());

    let dto = UpdateConfigCategoryDTO {
        id: Uuid::new_v4(),
        category_code: None,
        category_name: None,
        icon: None,
        color: None,
        order_num: None,
        remark: None,
        category_desc: None,
        is_builtin: None,
    };

    let err = svc.update(dto).await.unwrap_err();
    assert_eq!(err.kind(), infinity_error::ErrorKind::Database);
}

// =========================================================================
// Tests: page_list
// =========================================================================

#[tokio::test]
async fn page_list_success_maps_to_vo() {
    let repo = Arc::new(MockConfigCategoryRepository::new());
    let model = make_model();
    let model_id = model.id.as_uuid();
    let paginated = PaginatedData::try_new(vec![model], 1, 20, 1).unwrap();
    repo.set_page_list(MockBehavior::Ok(paginated)).await;
    let svc = make_service(repo.clone());

    let params = PaginationParams::new(1, 20, ConfigCategoryQueryDTO::default());
    let page = svc.page_list(params).await.unwrap();

    assert_eq!(page.total, 1);
    assert_eq!(page.page, 1);
    assert_eq!(page.items.len(), 1);
    assert_eq!(page.items[0].id, model_id);
}

#[tokio::test]
async fn page_list_empty_result() {
    let repo = Arc::new(MockConfigCategoryRepository::new());
    repo.set_page_list(MockBehavior::Ok(make_empty_paginated()))
        .await;
    let svc = make_service(repo.clone());

    let params = PaginationParams::new(1, 20, ConfigCategoryQueryDTO::default());
    let page = svc.page_list(params).await.unwrap();

    assert_eq!(page.total, 0);
    assert_eq!(page.items.len(), 0);
}

#[tokio::test]
async fn page_list_repo_error_propagates() {
    let repo = Arc::new(MockConfigCategoryRepository::new());
    repo.set_page_list(MockBehavior::Err(InfinityError::internal("查询异常")))
        .await;
    let svc = make_service(repo.clone());

    let params = PaginationParams::new(1, 20, ConfigCategoryQueryDTO::default());
    let err = svc.page_list(params).await.unwrap_err();
    assert_eq!(err.kind(), infinity_error::ErrorKind::Internal);
}

#[tokio::test]
async fn page_list_with_sort_params() {
    let repo = Arc::new(MockConfigCategoryRepository::new());
    repo.set_page_list(MockBehavior::Ok(make_empty_paginated()))
        .await;
    let svc = make_service(repo.clone());

    let params = PaginationParams::new(1, 10, ConfigCategoryQueryDTO::default())
        .with_sort(ConfigCategorySortField::CreateTime, SortOrder::Desc);
    let page = svc.page_list(params).await.unwrap();
    assert_eq!(page.items.len(), 0);
}

// =========================================================================
// Tests: get_by_id
// =========================================================================

#[tokio::test]
async fn get_by_id_exists_returns_vo() {
    let repo = Arc::new(MockConfigCategoryRepository::new());
    let model = make_model();
    let model_id = model.id.clone();
    repo.set_find_by_id(MockBehavior::Ok(Some(model.clone())))
        .await;
    let svc = make_service(repo.clone());

    let vo = svc.get_by_id(model_id).await.unwrap();
    assert_eq!(vo.id, model.id.as_uuid());
    assert_eq!(vo.category_code, model.category_code);
}

#[tokio::test]
async fn get_by_id_not_found_returns_error() {
    let repo = Arc::new(MockConfigCategoryRepository::new());
    repo.set_find_by_id(MockBehavior::Ok(None)).await;
    let svc = make_service(repo.clone());

    let err = svc.get_by_id(ConfigCategoryId::generate()).await.unwrap_err();
    assert_eq!(err.kind(), infinity_error::ErrorKind::NotFound);
    assert!(err.to_string().contains("config category not found"));
}

#[tokio::test]
async fn get_by_id_repo_error_propagates() {
    let repo = Arc::new(MockConfigCategoryRepository::new());
    repo.set_find_by_id(MockBehavior::Err(InfinityError::database("连接超时")))
        .await;
    let svc = make_service(repo.clone());

    let err = svc.get_by_id(ConfigCategoryId::generate()).await.unwrap_err();
    assert_eq!(err.kind(), infinity_error::ErrorKind::Database);
}

// =========================================================================
// Tests: check_exists
// =========================================================================

#[tokio::test]
async fn check_exists_code_exists() {
    let repo = Arc::new(MockConfigCategoryRepository::new());
    repo.set_check_exists(MockBehavior::Ok(ConfigCategoryExistsResult {
        category_code_exists: Some(true),
    }))
    .await;
    let svc = make_service(repo.clone());

    let dto = CheckConfigCategoryExistsDTO {
        category_code: Some("existing_code".into()),
    };
    let vo = svc.check_exists(dto).await.unwrap();
    assert_eq!(vo.category_code_exists, Some(true));
}

#[tokio::test]
async fn check_exists_code_not_exists() {
    let repo = Arc::new(MockConfigCategoryRepository::new());
    repo.set_check_exists(MockBehavior::Ok(ConfigCategoryExistsResult {
        category_code_exists: Some(false),
    }))
    .await;
    let svc = make_service(repo.clone());

    let dto = CheckConfigCategoryExistsDTO {
        category_code: Some("new_code".into()),
    };
    let vo = svc.check_exists(dto).await.unwrap();
    assert_eq!(vo.category_code_exists, Some(false));
}

#[tokio::test]
async fn check_exists_repo_error_propagates() {
    let repo = Arc::new(MockConfigCategoryRepository::new());
    repo.set_check_exists(MockBehavior::Err(InfinityError::database("查询失败")))
        .await;
    let svc = make_service(repo.clone());

    let dto = CheckConfigCategoryExistsDTO {
        category_code: Some("test_code".into()),
    };
    let err = svc.check_exists(dto).await.unwrap_err();
    assert_eq!(err.kind(), infinity_error::ErrorKind::Database);
}

#[tokio::test]
async fn check_exists_when_none_returns_none() {
    let repo = Arc::new(MockConfigCategoryRepository::new());
    repo.set_check_exists(MockBehavior::Ok(ConfigCategoryExistsResult {
        category_code_exists: None,
    }))
    .await;
    let svc = make_service(repo.clone());

    let dto = CheckConfigCategoryExistsDTO {
        category_code: Some("test_code".into()),
    };
    let vo = svc.check_exists(dto).await.unwrap();
    assert_eq!(vo.category_code_exists, None);
}

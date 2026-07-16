//! 集成测试：ConfigCategoryRepositoryImpl
//!
//! 需要可用的 PostgreSQL 测试数据库。通过以下环境变量配置：
//! - `TEST_DATABASE_URL`：测试数据库连接串，默认为 `postgres://postgres:postgres@localhost:5432/infinity_test`
//!
//! 测试执行前会自动运行 migrations，测试结束后清理数据。

use admin::domain::dto::config_category::{ConfigCategoryQueryDTO, ConfigCategorySortField};
use admin::domain::types::ids::ConfigCategoryId;
use admin::repository::config_category_repository::{
    ConfigCategoryRepository, NewConfigCategory, UpdateConfigCategory,
};
use admin::repository::postgres::config_category_repository_impl::ConfigCategoryRepositoryImpl;
use infinity_database::pool::Database;
use infinity_web::{PaginationParams, SortOrder};
use sqlx::PgPool;
use tokio::sync::OnceCell;

/// 全局共享的数据库连接池（只初始化一次）
static TEST_DB: OnceCell<Option<PgPool>> = OnceCell::const_new();

/// 初始化测试数据库，运行迁移。如果连接失败返回 None。
async fn setup_db() -> Option<PgPool> {
    let pool = TEST_DB
        .get_or_init(|| async {
            let url = std::env::var("TEST_DATABASE_URL").unwrap_or_else(|_| {
                "postgres://postgres:postgres@localhost:5432/infinity_test".into()
            });

            match Database::connect_with(&url, 5).await {
                Ok(db) => {
                    // 运行迁移
                    if let Err(e) = db.migrate("migrations").await {
                        eprintln!("⚠ 迁移失败（可能已是最新）: {e}");
                    }
                    eprintln!("✓ 测试数据库已连接: {url}");
                    Some(db.pool().clone())
                }
                Err(e) => {
                    eprintln!("✗ 无法连接测试数据库: {e}");
                    eprintln!("  请确保 PostgreSQL 已运行且数据库已创建");
                    eprintln!("  设置 TEST_DATABASE_URL 指向测试数据库");
                    None
                }
            }
        })
        .await;

    pool.clone()
}

/// 创建仓储实例
fn repo(pool: &PgPool) -> ConfigCategoryRepositoryImpl {
    ConfigCategoryRepositoryImpl::new(pool.clone())
}

/// 清除测试过程中创建的分类（通过 soft_delete）
async fn cleanup(pool: &PgPool, ids: &[ConfigCategoryId]) {
    if ids.is_empty() {
        return;
    }
    let _ = repo(pool).soft_delete(ids).await;
}

// ==================== create 测试 ====================

#[tokio::test]
async fn test_create_returns_saved_category() {
    let pool = match setup_db().await {
        Some(p) => p,
        None => {
            eprintln!("跳过：无测试数据库");
            return;
        }
    };
    let r = repo(&pool);

    let new_cat = NewConfigCategory {
        category_code: format!("test_create_{}", uuid::Uuid::new_v4()),
        category_name: "测试创建".into(),
        icon: None,
        color: None,
        order_num: 1,
        remark: None,
        category_desc: Some("测试描述".into()),
        is_builtin: false,
    };

    let result = r.create(new_cat).await.expect("创建应成功");
    assert_eq!(result.category_name, "测试创建");
    assert_eq!(result.order_num, 1);
    assert!(!result.is_builtin);
    assert!(!result.is_deleted);

    cleanup(&pool, &[result.id.clone()]).await;
}

#[tokio::test]
async fn test_create_with_all_fields() {
    let pool = match setup_db().await {
        Some(p) => p,
        None => {
            eprintln!("跳过：无测试数据库");
            return;
        }
    };
    let r = repo(&pool);

    let code = format!("test_full_{}", uuid::Uuid::new_v4());
    let new_cat = NewConfigCategory {
        category_code: code.clone(),
        category_name: "完整字段测试".into(),
        icon: Some("icon-abc".into()),
        color: Some("#FF0000".into()),
        order_num: 99,
        remark: Some("备注信息".into()),
        category_desc: Some("完整描述".into()),
        is_builtin: true,
    };

    let result = r.create(new_cat).await.expect("创建应成功");
    assert_eq!(result.category_code, code);
    assert_eq!(result.category_name, "完整字段测试");
    assert_eq!(result.icon.as_deref(), Some("icon-abc"));
    assert_eq!(result.color.as_deref(), Some("#FF0000"));
    assert_eq!(result.order_num, 99);
    assert_eq!(result.remark.as_deref(), Some("备注信息"));
    assert_eq!(result.category_desc.as_deref(), Some("完整描述"));
    assert!(result.is_builtin);

    cleanup(&pool, &[result.id.clone()]).await;
}

// ==================== find_by_id 测试 ====================

#[tokio::test]
async fn test_find_by_id_existing() {
    let pool = match setup_db().await {
        Some(p) => p,
        None => {
            eprintln!("跳过：无测试数据库");
            return;
        }
    };
    let r = repo(&pool);

    let created = r
        .create(NewConfigCategory {
            category_code: format!("test_find_{}", uuid::Uuid::new_v4()),
            category_name: "查找测试".into(),
            icon: None,
            color: None,
            order_num: 1,
            remark: None,
            category_desc: None,
            is_builtin: false,
        })
        .await
        .expect("创建应成功");

    let found = r
        .find_by_id(&created.id)
        .await
        .expect("查询应成功")
        .expect("应找到记录");

    assert_eq!(found.id, created.id);
    assert_eq!(found.category_name, "查找测试");

    cleanup(&pool, &[created.id.clone()]).await;
}

#[tokio::test]
async fn test_find_by_id_not_existing() {
    let pool = match setup_db().await {
        Some(p) => p,
        None => {
            eprintln!("跳过：无测试数据库");
            return;
        }
    };
    let r = repo(&pool);

    let fake_id = ConfigCategoryId::generate();
    let result = r.find_by_id(&fake_id).await.expect("查询应成功");
    assert!(result.is_none(), "不存在的 ID 应返回 None");
}

#[tokio::test]
async fn test_find_by_id_skips_deleted() {
    let pool = match setup_db().await {
        Some(p) => p,
        None => {
            eprintln!("跳过：无测试数据库");
            return;
        }
    };
    let r = repo(&pool);

    let created = r
        .create(NewConfigCategory {
            category_code: format!("test_deleted_find_{}", uuid::Uuid::new_v4()),
            category_name: "已删除查找".into(),
            icon: None,
            color: None,
            order_num: 1,
            remark: None,
            category_desc: None,
            is_builtin: false,
        })
        .await
        .expect("创建应成功");

    // 软删除
    r.soft_delete(&[created.id.clone()])
        .await
        .expect("软删除应成功");

    // 再查询应找不到
    let found = r.find_by_id(&created.id).await.expect("查询应成功");
    assert!(found.is_none(), "已删除记录不应被查询到");

    cleanup(&pool, &[created.id.clone()]).await;
}

// ==================== update_by_id 测试 ====================

#[tokio::test]
async fn test_update_by_id_success() {
    let pool = match setup_db().await {
        Some(p) => p,
        None => {
            eprintln!("跳过：无测试数据库");
            return;
        }
    };
    let r = repo(&pool);

    let created = r
        .create(NewConfigCategory {
            category_code: format!("test_upd_{}", uuid::Uuid::new_v4()),
            category_name: "更新前".into(),
            icon: None,
            color: None,
            order_num: 1,
            remark: None,
            category_desc: None,
            is_builtin: false,
        })
        .await
        .expect("创建应成功");

    let updated = r
        .update_by_id(UpdateConfigCategory {
            id: created.id.clone(),
            category_code: Some(format!("test_upd_new_{}", uuid::Uuid::new_v4())),
            category_name: Some("更新后".into()),
            icon: Some("new-icon".into()),
            color: Some("#00FF00".into()),
            order_num: Some(10),
            remark: Some("更新备注".into()),
            category_desc: Some("更新描述".into()),
            is_builtin: Some(true),
        })
        .await
        .expect("更新应成功")
        .expect("应返回更新后记录");

    assert_eq!(updated.category_name, "更新后");
    assert_eq!(updated.order_num, 10);
    assert_eq!(updated.icon.as_deref(), Some("new-icon"));
    assert!(!updated.is_deleted);

    cleanup(&pool, &[created.id.clone()]).await;
}

#[tokio::test]
async fn test_update_by_id_not_found() {
    let pool = match setup_db().await {
        Some(p) => p,
        None => {
            eprintln!("跳过：无测试数据库");
            return;
        }
    };
    let r = repo(&pool);

    let fake_id = ConfigCategoryId::generate();
    let result = r
        .update_by_id(UpdateConfigCategory {
            id: fake_id,
            category_code: Some("noop".into()),
            category_name: Some("不会更新".into()),
            icon: None,
            color: None,
            order_num: Some(1),
            remark: None,
            category_desc: None,
            is_builtin: Some(false),
        })
        .await
        .expect("查询应成功");
    assert!(result.is_none(), "不存在的 ID 更新应返回 None");
}

#[tokio::test]
async fn test_update_by_id_skips_deleted() {
    let pool = match setup_db().await {
        Some(p) => p,
        None => {
            eprintln!("跳过：无测试数据库");
            return;
        }
    };
    let r = repo(&pool);

    let created = r
        .create(NewConfigCategory {
            category_code: format!("test_upd_del_{}", uuid::Uuid::new_v4()),
            category_name: "已删待更新".into(),
            icon: None,
            color: None,
            order_num: 1,
            remark: None,
            category_desc: None,
            is_builtin: false,
        })
        .await
        .expect("创建应成功");

    // 软删除
    r.soft_delete(&[created.id.clone()])
        .await
        .expect("软删除应成功");

    // 尝试更新已删除的记录
    let result = r
        .update_by_id(UpdateConfigCategory {
            id: created.id.clone(),
            category_code: Some("should_fail".into()),
            category_name: Some("不应更新".into()),
            icon: None,
            color: None,
            order_num: Some(1),
            remark: None,
            category_desc: None,
            is_builtin: Some(false),
        })
        .await
        .expect("查询应成功");
    assert!(result.is_none(), "已删除记录不应被更新");

    cleanup(&pool, &[created.id.clone()]).await;
}

// ==================== soft_delete 测试 ====================

#[tokio::test]
async fn test_soft_delete_single() {
    let pool = match setup_db().await {
        Some(p) => p,
        None => {
            eprintln!("跳过：无测试数据库");
            return;
        }
    };
    let r = repo(&pool);

    let created = r
        .create(NewConfigCategory {
            category_code: format!("test_del_{}", uuid::Uuid::new_v4()),
            category_name: "待删除".into(),
            icon: None,
            color: None,
            order_num: 1,
            remark: None,
            category_desc: None,
            is_builtin: false,
        })
        .await
        .expect("创建应成功");

    let affected = r
        .soft_delete(&[created.id.clone()])
        .await
        .expect("软删除应成功");
    assert_eq!(affected, 1, "应影响 1 行");

    // 验证已删除
    let found = r.find_by_id(&created.id).await.expect("查询应成功");
    assert!(found.is_none(), "软删除后不可查");

    cleanup(&pool, &[]).await;
}

#[tokio::test]
async fn test_soft_delete_multiple() {
    let pool = match setup_db().await {
        Some(p) => p,
        None => {
            eprintln!("跳过：无测试数据库");
            return;
        }
    };
    let r = repo(&pool);

    let prefix = format!("test_multi_del_{}", uuid::Uuid::new_v4());
    let mut ids = Vec::new();
    for i in 0..3 {
        let created = r
            .create(NewConfigCategory {
                category_code: format!("{prefix}_{i}"),
                category_name: format!("批量删除{i}"),
                icon: None,
                color: None,
                order_num: i,
                remark: None,
                category_desc: None,
                is_builtin: false,
            })
            .await
            .expect("创建应成功");
        ids.push(created.id);
    }

    let affected = r.soft_delete(&ids).await.expect("批量软删除应成功");
    assert_eq!(affected, 3, "应影响 3 行");
}

#[tokio::test]
async fn test_soft_delete_empty_list() {
    let pool = match setup_db().await {
        Some(p) => p,
        None => {
            eprintln!("跳过：无测试数据库");
            return;
        }
    };
    let r = repo(&pool);

    let affected = r.soft_delete(&[]).await.expect("空列表删除应成功");
    assert_eq!(affected, 0, "空列表应返回 0");
}

#[tokio::test]
async fn test_soft_delete_already_deleted() {
    let pool = match setup_db().await {
        Some(p) => p,
        None => {
            eprintln!("跳过：无测试数据库");
            return;
        }
    };
    let r = repo(&pool);

    let created = r
        .create(NewConfigCategory {
            category_code: format!("test_double_del_{}", uuid::Uuid::new_v4()),
            category_name: "二次删除".into(),
            icon: None,
            color: None,
            order_num: 1,
            remark: None,
            category_desc: None,
            is_builtin: false,
        })
        .await
        .expect("创建应成功");

    // 第一次删除
    let affected1 = r
        .soft_delete(&[created.id.clone()])
        .await
        .expect("第一次删除应成功");
    assert_eq!(affected1, 1);

    // 第二次删除不应再影响行（is_deleted = FALSE 条件排除）
    let affected2 = r
        .soft_delete(&[created.id.clone()])
        .await
        .expect("第二次删除应成功");
    assert_eq!(affected2, 0, "已删除再次删除应不影响行");
}

// ==================== page_list 测试 ====================

#[tokio::test]
async fn test_page_list_basic_pagination() {
    let pool = match setup_db().await {
        Some(p) => p,
        None => {
            eprintln!("跳过：无测试数据库");
            return;
        }
    };
    let r = repo(&pool);

    let prefix = format!("test_page_{}", uuid::Uuid::new_v4());
    let mut ids = Vec::new();
    for i in 0..10 {
        let created = r
            .create(NewConfigCategory {
                category_code: format!("{prefix}_{i:02}"),
                category_name: format!("分页测试{i:02}"),
                icon: None,
                color: None,
                order_num: i,
                remark: None,
                category_desc: None,
                is_builtin: false,
            })
            .await
            .expect("创建应成功");
        ids.push(created.id);
    }

    // 第 1 页，每页 3 条
    let params = PaginationParams::<_, ConfigCategorySortField>::new(1, 3, Default::default());
    let page = r.page_list(params).await.expect("分页查询应成功");

    assert_eq!(page.items.len(), 3);
    assert!(page.total >= 10);
    assert_eq!(page.page, 1);
    assert_eq!(page.page_size, 3);

    // 第 2 页应有数据
    let params2 = PaginationParams::<_, ConfigCategorySortField>::new(2, 3, Default::default());
    let page2 = r.page_list(params2).await.expect("分页查询应成功");
    assert_eq!(page2.items.len(), 3);
    assert_eq!(page2.page, 2);

    cleanup(&pool, &ids).await;
}

#[tokio::test]
async fn test_page_list_with_category_code_filter() {
    let pool = match setup_db().await {
        Some(p) => p,
        None => {
            eprintln!("跳过：无测试数据库");
            return;
        }
    };
    let r = repo(&pool);

    let specific_code = format!("test_filter_{}", uuid::Uuid::new_v4());
    let created = r
        .create(NewConfigCategory {
            category_code: specific_code.clone(),
            category_name: "精确匹配".into(),
            icon: None,
            color: None,
            order_num: 1,
            remark: None,
            category_desc: None,
            is_builtin: false,
        })
        .await
        .expect("创建应成功");

    let filters = ConfigCategoryQueryDTO {
        category_code: Some(specific_code.clone()),
        category_name: None,
    };
    let params = PaginationParams::<_, ConfigCategorySortField>::new(1, 20, filters);
    let page = r.page_list(params).await.expect("分页查询应成功");

    assert_eq!(page.items.len(), 1, "应只匹配到一条");
    assert_eq!(page.items[0].category_code, specific_code);

    cleanup(&pool, &[created.id]).await;
}

#[tokio::test]
async fn test_page_list_with_category_name_like_filter() {
    let pool = match setup_db().await {
        Some(p) => p,
        None => {
            eprintln!("跳过：无测试数据库");
            return;
        }
    };
    let r = repo(&pool);

    let unique_name = format!("独特名称_{}", uuid::Uuid::new_v4());
    let created = r
        .create(NewConfigCategory {
            category_code: format!("test_like_{}", uuid::Uuid::new_v4()),
            category_name: unique_name.clone(),
            icon: None,
            color: None,
            order_num: 1,
            remark: None,
            category_desc: None,
            is_builtin: false,
        })
        .await
        .expect("创建应成功");

    // 使用部分名称模糊匹配
    let filters = ConfigCategoryQueryDTO {
        category_code: None,
        category_name: Some(unique_name[..6].to_string()),
    };
    let params = PaginationParams::<_, ConfigCategorySortField>::new(1, 20, filters);
    let page = r.page_list(params).await.expect("分页查询应成功");

    assert!(
        page.items.iter().any(|c| c.id == created.id),
        "模糊匹配应能找到该记录"
    );

    cleanup(&pool, &[created.id]).await;
}

#[tokio::test]
async fn test_page_list_with_sort() {
    let pool = match setup_db().await {
        Some(p) => p,
        None => {
            eprintln!("跳过：无测试数据库");
            return;
        }
    };
    let r = repo(&pool);

    let prefix = format!("test_sort_{}", uuid::Uuid::new_v4());
    let mut ids = Vec::new();
    for i in 0..5 {
        let created = r
            .create(NewConfigCategory {
                category_code: format!("{prefix}_{i:02}"),
                category_name: format!("排序测试{i}"),
                icon: None,
                color: None,
                order_num: i,
                remark: None,
                category_desc: None,
                is_builtin: false,
            })
            .await
            .expect("创建应成功");
        ids.push(created.id);
    }

    // 按 create_time 降序排序
    let params = PaginationParams::<_, ConfigCategorySortField>::new(1, 5, Default::default())
        .with_sort(ConfigCategorySortField::CreateTime, SortOrder::Desc);
    let page = r.page_list(params).await.expect("分页查询应成功");

    assert_eq!(page.items.len(), 5, "应返回 5 条数据");

    // 验证降序排序：越晚创建的时间越大
    for i in 1..page.items.len() {
        assert!(
            page.items[i - 1].created_at >= page.items[i].created_at,
            "降序排列应满足 created_at 递减"
        );
    }

    cleanup(&pool, &ids).await;
}

#[tokio::test]
async fn test_page_list_empty_filter_no_results() {
    let pool = match setup_db().await {
        Some(p) => p,
        None => {
            eprintln!("跳过：无测试数据库");
            return;
        }
    };
    let r = repo(&pool);

    let filters = ConfigCategoryQueryDTO {
        category_code: Some(format!("nonexistent_{}", uuid::Uuid::new_v4())),
        category_name: None,
    };
    let params = PaginationParams::<_, ConfigCategorySortField>::new(1, 20, filters);
    let page = r.page_list(params).await.expect("分页查询应成功");

    assert!(page.items.is_empty(), "不存在的分类编码应返回空");
    assert_eq!(page.total, 0, "总数应为 0");
}

// ==================== 边界/异常测试 ====================

#[tokio::test]
async fn test_soft_delete_with_invalid_uuid() {
    let pool = match setup_db().await {
        Some(p) => p,
        None => {
            eprintln!("跳过：无测试数据库");
            return;
        }
    };
    let r = repo(&pool);

    // 使用随机生成的 ID（数据库中不存在）
    let fake_id = ConfigCategoryId::generate();
    let affected = r
        .soft_delete(&[fake_id])
        .await
        .expect("删除不存在的 ID 应成功");
    assert_eq!(affected, 0, "不存在的 ID 不应影响任何行");
}

#[tokio::test]
async fn test_page_list_large_page_number() {
    let pool = match setup_db().await {
        Some(p) => p,
        None => {
            eprintln!("跳过：无测试数据库");
            return;
        }
    };
    let r = repo(&pool);

    // 请求一个很大的页码，应返回空
    let params = PaginationParams::<_, ConfigCategorySortField>::new(99999, 20, Default::default());
    let page = r.page_list(params).await.expect("分页查询应成功");

    assert!(page.items.is_empty(), "超大纲页码应返回空");
    assert!(page.page == 99999);
}

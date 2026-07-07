//! `AdminRepository` 集成测试（需要可用的 PostgreSQL）。
//!
//! 数据库地址取自 `DATABASE_URL` 环境变量，未设置时回退到默认
//! `postgres://postgres:postgres@localhost/infinity`。运行前请确保该 PG 可用，例如：
//!
//! ```bash
//! DATABASE_URL=postgres://user:pass@localhost/infinity \
//!   cargo test -p infinity-database
//! ```

use infinity_database::Database;
use infinity_database::repository::{AdminRecord, AdminRepository};

/// 默认连接串，与 `DatabaseConfig::default()` 保持一致。
const DEFAULT_URL: &str = "postgres://postgres:postgres@localhost/infinity";

/// 连接数据库并运行迁移。
///
/// 直接用 `DATABASE_URL` 而非 `Config::from_dir`，避免其全局单例在多测试间冲突。
async fn setup() -> Database {
    let url = std::env::var("DATABASE_URL").unwrap_or_else(|_| DEFAULT_URL.to_owned());
    let db = Database::connect_with(&url, 5)
        .await
        .expect("connect database");
    db.migrate().await.expect("run migrations");
    db
}

#[tokio::test]
async fn insert_then_find_by_id_returns_record() {
    let db = setup().await;
    let repo = AdminRepository::new(&db);

    let record = AdminRecord {
        id: "test-admin-1".to_owned(),
        tenant_id: "test-tenant-1".to_owned(),
        username: "test-admin-1-user".to_owned(),
    };

    // 幂等：先清理同 ID 记录，保证测试可重复运行。
    sqlx::query("DELETE FROM admins WHERE id = $1")
        .bind(&record.id)
        .execute(db.pool())
        .await
        .expect("cleanup existing row");

    repo.insert(&record).await.expect("insert admin");

    let found = repo.find_by_id(&record.id).await.expect("find admin");
    assert_eq!(found, Some(record));
}

#[tokio::test]
async fn find_by_id_missing_returns_none() {
    let db = setup().await;
    let repo = AdminRepository::new(&db);

    let found = repo
        .find_by_id("does-not-exist-zzz")
        .await
        .expect("find admin");
    assert_eq!(found, None);
}

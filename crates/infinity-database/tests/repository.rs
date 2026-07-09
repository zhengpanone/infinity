//! Integration tests for `AdminRepository`.
//!
//! These tests require a reachable PostgreSQL database. They are skipped by
//! default unless `DATABASE_URL` is set, so normal workspace test runs do not
//! fail on machines without a local database.
//!
//! ```powershell
//! $env:DATABASE_URL="postgres://user:pass@localhost/infinity"
//! cargo test -p infinity-database --test repository
//! ```

use infinity_database::Database;
use infinity_database::repository::{AdminRecord, AdminRepository};

async fn setup() -> Option<Database> {
    let Ok(url) = std::env::var("DATABASE_URL") else {
        eprintln!("skipping database integration test: DATABASE_URL is not set");
        return None;
    };

    Some(
        Database::connect_with(&url, 5)
            .await
            .expect("connect database"),
    )
}

#[tokio::test]
async fn insert_then_find_by_id_returns_record() {
    let Some(db) = setup().await else {
        return;
    };
    let repo = AdminRepository::new(&db);

    let record = AdminRecord {
        id: "test-admin-1".to_owned(),
        tenant_id: "test-tenant-1".to_owned(),
        username: "test-admin-1-user".to_owned(),
    };

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
    let Some(db) = setup().await else {
        return;
    };
    let repo = AdminRepository::new(&db);

    let found = repo
        .find_by_id("does-not-exist-zzz")
        .await
        .expect("find admin");
    assert_eq!(found, None);
}

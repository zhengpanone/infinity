//! 仓储层：领域实体的数据库读写。
//!
//! 记录类型（如 [`AdminRecord`]）只承载数据库列，不含领域行为；与领域模型的
//! 映射由应用层完成，从而让本 crate 不依赖 `infinity-common`。

use infinity_error::{ErrorKind, Result, ResultExt};
use sqlx::PgPool;

use crate::Database;

/// 管理员数据库记录（对应 `admins` 表）。
#[derive(Debug, Clone, PartialEq, Eq, sqlx::FromRow)]
pub struct AdminRecord {
    /// 全局唯一用户 ID。
    pub id: String,
    /// 所属租户 ID。
    pub tenant_id: String,
    /// 登录用户名（唯一）。
    pub username: String,
}

/// 管理员仓储，封装 `admins` 表的读写。
pub struct AdminRepository<'a> {
    pool: &'a PgPool,
}

impl<'a> AdminRepository<'a> {
    /// 基于数据库句柄创建仓储。
    pub fn new(db: &'a Database) -> Self {
        Self { pool: db.pool() }
    }

    /// 按 ID 查询管理员；不存在返回 `None`。
    pub async fn find_by_id(&self, id: &str) -> Result<Option<AdminRecord>> {
        sqlx::query_as::<_, AdminRecord>("SELECT id, tenant_id, username FROM admins WHERE id = $1")
            .bind(id)
            .fetch_optional(self.pool)
            .await
            .with_context(ErrorKind::Database, || {
                format!("failed to query admin {id}")
            })
    }

    /// 插入一条管理员记录。
    pub async fn insert(&self, record: &AdminRecord) -> Result<()> {
        sqlx::query("INSERT INTO admins (id, tenant_id, username) VALUES ($1, $2, $3)")
            .bind(&record.id)
            .bind(&record.tenant_id)
            .bind(&record.username)
            .execute(self.pool)
            .await
            .with_context(ErrorKind::Database, || {
                format!("failed to insert admin {}", record.id)
            })?;
        Ok(())
    }
}

//! 数据库健康检查。

use infinity_error::{ErrorKind, Result, ResultExt};

use crate::Database;

/// 对数据库执行一次轻量探活查询（`SELECT 1`）。
///
/// 供上层健康检查端点使用；失败转换为
/// [`ErrorKind::Database`](infinity_error::ErrorKind::Database)。
pub async fn ping(db: &Database) -> Result<()> {
    sqlx::query("SELECT 1")
        .execute(db.pool())
        .await
        .context(ErrorKind::Database, "database ping failed")?;
    Ok(())
}

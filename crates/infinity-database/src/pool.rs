//! PostgreSQL 连接池与迁移。

use std::time::Duration;

use infinity_config::config::DatabaseConfig;
use infinity_error::{ErrorKind, Result, ResultExt};
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;

/// 获取连接的默认超时，避免地址不可达时长时间挂起。
const ACQUIRE_TIMEOUT: Duration = Duration::from_secs(10);

/// 数据库句柄，内部持有一个 `sqlx` 连接池。
///
/// 通过 [`Database::connect`] 创建后可克隆廉价共享（`PgPool` 内部为 `Arc`）。
#[derive(Debug, Clone)]
pub struct Database {
    pool: PgPool,
}

impl Database {
    /// 按配置创建连接池。
    ///
    /// 连接失败（URL 无效、数据库不可达等）在此处转换为
    /// [`ErrorKind::Database`](infinity_error::ErrorKind::Database)。
    pub async fn connect(cfg: &DatabaseConfig) -> Result<Self> {
        Self::connect_with(&cfg.url, cfg.max_connections).await
    }

    /// 以显式连接串与连接数上限创建连接池。
    ///
    /// 供测试或不经 [`DatabaseConfig`] 的场景直接使用。
    pub async fn connect_with(url: &str, max_connections: u32) -> Result<Self> {
        let pool = PgPoolOptions::new()
            .max_connections(max_connections)
            .acquire_timeout(ACQUIRE_TIMEOUT)
            .connect(url)
            .await
            .with_context(ErrorKind::Database, || {
                format!("failed to connect database (max_connections={max_connections})")
            })?;

        Ok(Self { pool })
    }

    /// 返回底层连接池引用，供仓储层构造查询。
    pub fn pool(&self) -> &PgPool {
        &self.pool
    }

    /// 运行 `./migrations` 下的全部迁移（幂等）。
    pub async fn migrate(&self) -> Result<()> {
        sqlx::migrate!("./migrations")
            .run(&self.pool)
            .await
            .context(ErrorKind::Database, "database migration failed")?;
        Ok(())
    }
}

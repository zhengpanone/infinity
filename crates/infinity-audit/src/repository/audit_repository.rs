//! 审计日志仓储:抽象接口、PostgreSQL 实现与内存实现。

use std::sync::Mutex;

use async_trait::async_trait;
use infinity_error::{ErrorKind, Result, ResultExt};
use sqlx::{PgPool, Row};

use crate::domain::{AuditLog, AuditQuery, AuditRecord};

/// 查询条数的默认上限,防止无 limit 查询拖垮数据库。
const DEFAULT_QUERY_LIMIT: u32 = 100;

/// 审计日志仓储接口。
///
/// 实现方负责持久化与哈希链维护;调用方(通常是
/// [`AuditService`](crate::service::AuditService))不关心存储介质。
#[async_trait]
pub trait AuditRepository: Send + Sync {
    /// 追加一条审计日志,返回落库后的链记录。
    ///
    /// 实现必须保证哈希链连续:新记录的 `previous_hash` 等于当前最后一条
    /// 记录的链哈希(空链为 `""`)。
    async fn append(&self, log: &AuditLog) -> Result<AuditRecord>;

    /// 按条件查询审计日志,按发生时间倒序返回。
    async fn query(&self, query: &AuditQuery) -> Result<Vec<AuditLog>>;
}

/// PostgreSQL 审计仓储(对应 `audit_logs` 表)。
///
/// 建表参考:
///
/// ```sql
/// CREATE TABLE audit_logs (
///     seq           BIGSERIAL PRIMARY KEY,
///     log_id        UUID        NOT NULL UNIQUE,
///     actor_id      TEXT        NOT NULL,
///     tenant_id     TEXT        NOT NULL,
///     action        TEXT        NOT NULL,
///     result        TEXT        NOT NULL,
///     payload       JSONB       NOT NULL,
///     payload_hash  TEXT        NOT NULL,
///     previous_hash TEXT        NOT NULL,
///     occurred_at   TIMESTAMPTZ NOT NULL
/// );
/// CREATE INDEX idx_audit_logs_tenant_time ON audit_logs (tenant_id, occurred_at DESC);
/// ```
pub struct PgAuditRepository {
    pool: PgPool,
}

impl PgAuditRepository {
    /// 基于连接池创建仓储。
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl AuditRepository for PgAuditRepository {
    async fn append(&self, log: &AuditLog) -> Result<AuditRecord> {
        let payload = serde_json::to_value(log)
            .context(ErrorKind::Internal, "serialize audit log failed")?;
        let payload_hash = log.payload_hash();

        // 在单个事务里读取链尾并插入,配合表锁保证并发下链哈希连续。
        let mut tx = self
            .pool
            .begin()
            .await
            .context(ErrorKind::Database, "begin audit transaction failed")?;

        sqlx::query("LOCK TABLE audit_logs IN EXCLUSIVE MODE")
            .execute(&mut *tx)
            .await
            .context(ErrorKind::Database, "lock audit_logs failed")?;

        let previous_hash: String = sqlx::query(
            "SELECT seq, payload_hash, previous_hash FROM audit_logs ORDER BY seq DESC LIMIT 1",
        )
        .fetch_optional(&mut *tx)
        .await
        .context(ErrorKind::Database, "load audit chain tail failed")?
        .map(|row| {
            AuditRecord {
                seq: row.get("seq"),
                log_id: log.id, // chain_hash 不使用 log_id,占位即可
                payload_hash: row.get("payload_hash"),
                previous_hash: row.get("previous_hash"),
            }
            .chain_hash()
        })
        .unwrap_or_default();

        let seq: i64 = sqlx::query_scalar(
            "INSERT INTO audit_logs \
             (log_id, actor_id, tenant_id, action, result, payload, payload_hash, previous_hash, occurred_at) \
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9) RETURNING seq",
        )
        .bind(log.id)
        .bind(&log.actor_id)
        .bind(&log.tenant_id)
        .bind(log.action.code())
        .bind(log.result.code())
        .bind(&payload)
        .bind(&payload_hash)
        .bind(&previous_hash)
        .bind(log.occurred_at)
        .fetch_one(&mut *tx)
        .await
        .with_context(ErrorKind::Database, || {
            format!("insert audit log {} failed", log.id)
        })?;

        tx.commit()
            .await
            .context(ErrorKind::Database, "commit audit transaction failed")?;

        Ok(AuditRecord {
            seq,
            log_id: log.id,
            payload_hash,
            previous_hash,
        })
    }

    async fn query(&self, query: &AuditQuery) -> Result<Vec<AuditLog>> {
        let limit = i64::from(query.limit.unwrap_or(DEFAULT_QUERY_LIMIT));

        let rows = sqlx::query(
            "SELECT payload FROM audit_logs \
             WHERE ($1::TEXT IS NULL OR actor_id = $1) \
               AND ($2::TEXT IS NULL OR tenant_id = $2) \
               AND ($3::TEXT IS NULL OR action = $3) \
               AND ($4::TEXT IS NULL OR result = $4) \
               AND ($5::TIMESTAMPTZ IS NULL OR occurred_at >= $5) \
               AND ($6::TIMESTAMPTZ IS NULL OR occurred_at < $6) \
             ORDER BY occurred_at DESC LIMIT $7",
        )
        .bind(&query.actor_id)
        .bind(&query.tenant_id)
        .bind(query.action.map(|a| a.code()))
        .bind(query.result.map(|r| r.code()))
        .bind(query.from)
        .bind(query.to)
        .bind(limit)
        .fetch_all(&self.pool)
        .await
        .context(ErrorKind::Database, "query audit logs failed")?;

        rows.into_iter()
            .map(|row| {
                serde_json::from_value(row.get("payload"))
                    .context(ErrorKind::Parse, "deserialize audit payload failed")
            })
            .collect()
    }
}

/// 内存审计仓储,供测试与本地开发使用。
///
/// 与 [`PgAuditRepository`] 遵循相同的哈希链语义。
#[derive(Default)]
pub struct InMemoryAuditRepository {
    entries: Mutex<Vec<(AuditRecord, AuditLog)>>,
}

impl InMemoryAuditRepository {
    /// 创建空仓储。
    pub fn new() -> Self {
        Self::default()
    }

    /// 返回全部链记录(按 seq 升序),供链校验测试使用。
    pub fn records(&self) -> Vec<AuditRecord> {
        self.entries
            .lock()
            .expect("audit store poisoned")
            .iter()
            .map(|(record, _)| record.clone())
            .collect()
    }
}

#[async_trait]
impl AuditRepository for InMemoryAuditRepository {
    async fn append(&self, log: &AuditLog) -> Result<AuditRecord> {
        let mut entries = self.entries.lock().expect("audit store poisoned");

        let previous_hash = entries
            .last()
            .map(|(record, _)| record.chain_hash())
            .unwrap_or_default();

        let record = AuditRecord {
            seq: entries.len() as i64 + 1,
            log_id: log.id,
            payload_hash: log.payload_hash(),
            previous_hash,
        };
        entries.push((record.clone(), log.clone()));
        Ok(record)
    }

    async fn query(&self, query: &AuditQuery) -> Result<Vec<AuditLog>> {
        let entries = self.entries.lock().expect("audit store poisoned");
        let limit = query.limit.unwrap_or(DEFAULT_QUERY_LIMIT) as usize;

        let mut logs: Vec<AuditLog> = entries
            .iter()
            .map(|(_, log)| log)
            .filter(|log| {
                query.actor_id.as_ref().is_none_or(|v| &log.actor_id == v)
                    && query.tenant_id.as_ref().is_none_or(|v| &log.tenant_id == v)
                    && query.action.is_none_or(|v| log.action == v)
                    && query.result.is_none_or(|v| log.result == v)
                    && query.from.is_none_or(|v| log.occurred_at >= v)
                    && query.to.is_none_or(|v| log.occurred_at < v)
            })
            .cloned()
            .collect();
        logs.sort_by_key(|log| std::cmp::Reverse(log.occurred_at));
        logs.truncate(limit);
        Ok(logs)
    }
}

#[cfg(test)]
mod tests {
    use chrono::{Duration, Utc};

    use super::*;
    use crate::domain::{AuditAction, AuditResult};

    fn log_at(actor: &str, action: AuditAction, minutes_ago: i64) -> AuditLog {
        AuditLog::builder(actor, "t-1", action, AuditResult::Succeeded)
            .occurred_at(Utc::now() - Duration::minutes(minutes_ago))
            .build()
    }

    #[tokio::test]
    async fn append_builds_contiguous_hash_chain() {
        let repo = InMemoryAuditRepository::new();
        for i in 0..3 {
            repo.append(&log_at("u-1", AuditAction::Create, i))
                .await
                .unwrap();
        }

        let records = repo.records();
        assert_eq!(records.len(), 3);
        assert_eq!(records[0].previous_hash, "");
        assert_eq!(AuditRecord::verify_chain(&records), None);
    }

    #[tokio::test]
    async fn query_filters_and_orders_by_time_desc() {
        let repo = InMemoryAuditRepository::new();
        repo.append(&log_at("u-1", AuditAction::Create, 30))
            .await
            .unwrap();
        repo.append(&log_at("u-1", AuditAction::Delete, 10))
            .await
            .unwrap();
        repo.append(&log_at("u-2", AuditAction::Create, 20))
            .await
            .unwrap();

        let logs = repo
            .query(&AuditQuery {
                actor_id: Some("u-1".to_owned()),
                ..Default::default()
            })
            .await
            .unwrap();

        assert_eq!(logs.len(), 2);
        // 时间倒序:10 分钟前的 Delete 在前
        assert_eq!(logs[0].action, AuditAction::Delete);
        assert_eq!(logs[1].action, AuditAction::Create);
    }

    #[tokio::test]
    async fn query_respects_limit_and_time_range() {
        let repo = InMemoryAuditRepository::new();
        for i in 0..5 {
            repo.append(&log_at("u-1", AuditAction::Read, i * 10))
                .await
                .unwrap();
        }

        let limited = repo
            .query(&AuditQuery {
                limit: Some(2),
                ..Default::default()
            })
            .await
            .unwrap();
        assert_eq!(limited.len(), 2);

        let recent = repo
            .query(&AuditQuery {
                from: Some(Utc::now() - Duration::minutes(15)),
                ..Default::default()
            })
            .await
            .unwrap();
        assert_eq!(recent.len(), 2);
    }
}

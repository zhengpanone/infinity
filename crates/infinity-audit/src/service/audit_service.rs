//! 审计服务:根据落库策略记录审计日志。

use std::sync::Arc;

use infinity_error::Result;
use tracing::{error, warn};

use crate::domain::{AuditLog, AuditPolicy, AuditQuery, AuditRecord, policy_for, should_alert};
use crate::repository::AuditRepository;

/// 审计服务,业务侧记录审计日志的统一入口。
///
/// 落库策略由动作决定(见 [`policy_for`]):
///
/// - [`AuditPolicy::MustRecord`][]:高危操作,落库失败向上抛错,业务应随之失败;
/// - [`AuditPolicy::BestEffort`][]:普通操作,落库失败仅记 warn 日志,不影响业务。
#[derive(Clone)]
pub struct AuditService {
    repository: Arc<dyn AuditRepository>,
}

impl AuditService {
    /// 基于任意仓储实现创建服务。
    pub fn new(repository: Arc<dyn AuditRepository>) -> Self {
        Self { repository }
    }

    /// 按动作默认策略记录一条审计日志。
    ///
    /// `BestEffort` 下失败返回 `Ok(None)`;`MustRecord` 下失败返回 `Err`。
    /// 高危操作被拒绝时(见 [`should_alert`])额外输出 error 级告警日志,
    /// 供告警系统按日志级别订阅。
    pub async fn record(&self, log: AuditLog) -> Result<Option<AuditRecord>> {
        self.record_with_policy(log, None).await
    }

    /// 以显式策略记录审计日志,覆盖动作默认策略。
    pub async fn record_with_policy(
        &self,
        log: AuditLog,
        policy: Option<AuditPolicy>,
    ) -> Result<Option<AuditRecord>> {
        if should_alert(log.result, log.action) {
            error!(
                audit.action = log.action.code(),
                audit.actor_id = %log.actor_id,
                audit.tenant_id = %log.tenant_id,
                audit.target_id = %log.target_id,
                "sensitive action denied"
            );
        }

        let policy = policy.unwrap_or_else(|| policy_for(log.action));
        match self.repository.append(&log).await {
            Ok(record) => Ok(Some(record)),
            Err(err) => match policy {
                AuditPolicy::MustRecord => Err(err),
                AuditPolicy::BestEffort => {
                    warn!(
                        audit.action = log.action.code(),
                        audit.actor_id = %log.actor_id,
                        error = %err,
                        "best-effort audit write failed"
                    );
                    Ok(None)
                }
            },
        }
    }

    /// 按条件查询审计日志。
    pub async fn query(&self, query: &AuditQuery) -> Result<Vec<AuditLog>> {
        self.repository.query(query).await
    }
}

#[cfg(test)]
mod tests {
    use async_trait::async_trait;
    use infinity_error::InfinityError;

    use super::*;
    use crate::domain::{AuditAction, AuditResult};
    use crate::repository::InMemoryAuditRepository;

    /// 永远写失败的仓储,用于验证落库策略。
    struct FailingRepository;

    #[async_trait]
    impl AuditRepository for FailingRepository {
        async fn append(&self, _log: &AuditLog) -> Result<AuditRecord> {
            Err(InfinityError::database("audit store down"))
        }

        async fn query(&self, _query: &AuditQuery) -> Result<Vec<AuditLog>> {
            Ok(Vec::new())
        }
    }

    fn log(action: AuditAction) -> AuditLog {
        AuditLog::builder("u-1", "t-1", action, AuditResult::Succeeded).build()
    }

    #[tokio::test]
    async fn record_persists_and_returns_chain_record() {
        let service = AuditService::new(Arc::new(InMemoryAuditRepository::new()));

        let record = service.record(log(AuditAction::Create)).await.unwrap();
        assert!(record.is_some());

        let logs = service.query(&AuditQuery::default()).await.unwrap();
        assert_eq!(logs.len(), 1);
    }

    #[tokio::test]
    async fn best_effort_swallows_write_failure() {
        let service = AuditService::new(Arc::new(FailingRepository));

        let outcome = service.record(log(AuditAction::Read)).await.unwrap();
        assert!(outcome.is_none(), "BestEffort 失败应返回 Ok(None)");
    }

    #[tokio::test]
    async fn must_record_propagates_write_failure() {
        let service = AuditService::new(Arc::new(FailingRepository));

        let err = service.record(log(AuditAction::GrantRole)).await;
        assert!(err.is_err(), "MustRecord 失败应向上抛错");
    }

    #[tokio::test]
    async fn explicit_policy_overrides_default() {
        let service = AuditService::new(Arc::new(FailingRepository));

        // Read 默认 BestEffort,显式指定 MustRecord 后失败应抛错。
        let err = service
            .record_with_policy(log(AuditAction::Read), Some(AuditPolicy::MustRecord))
            .await;
        assert!(err.is_err());
    }
}

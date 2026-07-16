//! 审计日志实体与构建器。

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use uuid::Uuid;

use crate::domain::action::AuditAction;
use crate::domain::result::{AuditResult, FieldChange};

/// 一条完整的审计日志。
///
/// 通过 [`AuditLog::builder`] 构建;必填项为操作者、租户、动作与结果,
/// 其余字段按需补充。
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AuditLog {
    /// 日志唯一 ID(UUID v7,自带时间序)。
    pub id: Uuid,
    /// 操作者 ID。
    pub actor_id: String,
    /// 操作者显示名。
    pub actor_name: String,
    /// 操作者类型(如 `admin`、`user`、`system`)。
    pub actor_type: String,
    /// 所属租户 ID。
    pub tenant_id: String,
    /// 审计动作。
    pub action: AuditAction,
    /// 操作结果。
    pub result: AuditResult,
    /// 目标资源 ID。
    pub target_id: String,
    /// 目标资源显示名。
    pub target_name: String,
    /// 目标资源类型(如 `user`、`order`)。
    pub target_type: String,
    /// 面向人的操作描述。
    pub description: String,
    /// 字段级变更明细(更新类操作)。敏感字段须先经 [`mask`](crate::mask) 脱敏。
    pub changes: Vec<FieldChange>,
    /// 客户端 IP。
    pub ip: String,
    /// 客户端 User-Agent。
    pub user_agent: String,
    /// 请求 ID,用于与访问日志关联。
    pub request_id: String,
    /// 操作发生时间。
    pub occurred_at: DateTime<Utc>,
}

impl AuditLog {
    /// 创建构建器,必填项在此一次给齐。
    pub fn builder(
        actor_id: impl Into<String>,
        tenant_id: impl Into<String>,
        action: AuditAction,
        result: AuditResult,
    ) -> AuditLogBuilder {
        AuditLogBuilder {
            log: AuditLog {
                id: Uuid::now_v7(),
                actor_id: actor_id.into(),
                actor_name: String::new(),
                actor_type: String::new(),
                tenant_id: tenant_id.into(),
                action,
                result,
                target_id: String::new(),
                target_name: String::new(),
                target_type: String::new(),
                description: String::new(),
                changes: Vec::new(),
                ip: String::new(),
                user_agent: String::new(),
                request_id: String::new(),
                occurred_at: Utc::now(),
            },
        }
    }

    /// 计算日志内容的 SHA-256 摘要(十六进制),作为哈希链的 payload。
    ///
    /// 基于序列化后的 JSON 计算;同一条日志内容不变则摘要稳定。
    pub fn payload_hash(&self) -> String {
        // serde_json 对 struct 的字段序稳定(声明序),摘要可复现。
        let payload = serde_json::to_vec(self).unwrap_or_default();
        hex::encode(Sha256::digest(payload))
    }
}

/// [`AuditLog`] 的构建器。
#[derive(Debug)]
pub struct AuditLogBuilder {
    log: AuditLog,
}

impl AuditLogBuilder {
    /// 设置操作者显示名与类型。
    pub fn actor(mut self, name: impl Into<String>, actor_type: impl Into<String>) -> Self {
        self.log.actor_name = name.into();
        self.log.actor_type = actor_type.into();
        self
    }

    /// 设置目标资源。
    pub fn target(
        mut self,
        id: impl Into<String>,
        name: impl Into<String>,
        target_type: impl Into<String>,
    ) -> Self {
        self.log.target_id = id.into();
        self.log.target_name = name.into();
        self.log.target_type = target_type.into();
        self
    }

    /// 设置操作描述。
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.log.description = description.into();
        self
    }

    /// 追加一条字段变更。
    pub fn change(mut self, change: FieldChange) -> Self {
        self.log.changes.push(change);
        self
    }

    /// 设置请求来源信息。
    pub fn source(
        mut self,
        ip: impl Into<String>,
        user_agent: impl Into<String>,
        request_id: impl Into<String>,
    ) -> Self {
        self.log.ip = ip.into();
        self.log.user_agent = user_agent.into();
        self.log.request_id = request_id.into();
        self
    }

    /// 覆盖操作发生时间(默认为构建时刻)。
    pub fn occurred_at(mut self, at: DateTime<Utc>) -> Self {
        self.log.occurred_at = at;
        self
    }

    /// 完成构建。
    pub fn build(self) -> AuditLog {
        self.log
    }
}

/// 已落库的审计记录,携带哈希链字段以支持防篡改校验。
///
/// `payload_hash` 是日志内容摘要,`previous_hash` 指向上一条记录的
/// [`chain_hash`](AuditRecord::chain_hash),形成链式结构:篡改任一历史记录
/// 都会使其后所有记录的链哈希校验失败。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuditRecord {
    /// 落库自增序号(哈希链的链序)。
    pub seq: i64,
    /// 日志唯一 ID。
    pub log_id: Uuid,
    /// 日志内容摘要。
    pub payload_hash: String,
    /// 上一条记录的链哈希;首条记录为空串。
    pub previous_hash: String,
}

impl AuditRecord {
    /// 计算本条记录的链哈希:`SHA-256(seq:previous_hash:payload_hash)`。
    pub fn chain_hash(&self) -> String {
        let input = format!("{}:{}:{}", self.seq, self.previous_hash, self.payload_hash);
        hex::encode(Sha256::digest(input.as_bytes()))
    }

    /// 校验 `records` 是否构成完整未篡改的哈希链。
    ///
    /// 要求按 `seq` 升序传入;返回第一条断链记录的下标,全部通过返回 `None`。
    pub fn verify_chain(records: &[AuditRecord]) -> Option<usize> {
        let mut previous = String::new();
        for (i, record) in records.iter().enumerate() {
            if record.previous_hash != previous {
                return Some(i);
            }
            previous = record.chain_hash();
        }
        None
    }
}

/// 审计日志查询条件,全部字段可选,组合成 AND 过滤。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AuditQuery {
    /// 按操作者过滤。
    pub actor_id: Option<String>,
    /// 按租户过滤。
    pub tenant_id: Option<String>,
    /// 按动作过滤。
    pub action: Option<AuditAction>,
    /// 按结果过滤。
    pub result: Option<AuditResult>,
    /// 起始时间(含)。
    pub from: Option<DateTime<Utc>>,
    /// 截止时间(不含)。
    pub to: Option<DateTime<Utc>>,
    /// 返回条数上限,`None` 时由仓储实现取默认值。
    pub limit: Option<u32>,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_log() -> AuditLog {
        AuditLog::builder("u-1", "t-1", AuditAction::GrantRole, AuditResult::Succeeded)
            .actor("Alice", "admin")
            .target("u-2", "Bob", "user")
            .description("授予 Bob 管理员角色")
            .change(FieldChange::plain("role", "member", "admin"))
            .source("10.0.0.1", "curl/8", "req-1")
            .build()
    }

    #[test]
    fn builder_populates_all_fields() {
        let log = sample_log();
        assert_eq!(log.actor_name, "Alice");
        assert_eq!(log.target_id, "u-2");
        assert_eq!(log.changes.len(), 1);
        assert_eq!(log.request_id, "req-1");
    }

    #[test]
    fn payload_hash_is_stable_and_content_sensitive() {
        let log = sample_log();
        assert_eq!(log.payload_hash(), log.payload_hash());

        let mut tampered = log.clone();
        tampered.description = "篡改后的描述".to_owned();
        assert_ne!(log.payload_hash(), tampered.payload_hash());
    }

    fn build_chain(hashes: &[&str]) -> Vec<AuditRecord> {
        let mut records = Vec::new();
        let mut previous = String::new();
        for (i, payload) in hashes.iter().enumerate() {
            let record = AuditRecord {
                seq: i as i64 + 1,
                log_id: Uuid::now_v7(),
                payload_hash: (*payload).to_owned(),
                previous_hash: previous.clone(),
            };
            previous = record.chain_hash();
            records.push(record);
        }
        records
    }

    #[test]
    fn intact_chain_verifies() {
        let records = build_chain(&["h1", "h2", "h3"]);
        assert_eq!(AuditRecord::verify_chain(&records), None);
    }

    #[test]
    fn tampered_payload_breaks_chain_at_next_record() {
        let mut records = build_chain(&["h1", "h2", "h3"]);
        // 篡改第二条的内容:第三条的 previous_hash 不再匹配。
        records[1].payload_hash = "tampered".to_owned();
        assert_eq!(AuditRecord::verify_chain(&records), Some(2));
    }

    #[test]
    fn empty_chain_verifies() {
        assert_eq!(AuditRecord::verify_chain(&[]), None);
    }
}

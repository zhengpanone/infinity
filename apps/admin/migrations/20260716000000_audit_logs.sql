-- 审计日志表(infinity-audit PgAuditRepository)。
-- seq 为哈希链链序;previous_hash 指向上一条记录的链哈希,防篡改。
CREATE TABLE audit_logs
(
    seq           BIGSERIAL PRIMARY KEY,
    log_id        UUID        NOT NULL UNIQUE,
    actor_id      TEXT        NOT NULL,
    tenant_id     TEXT        NOT NULL,
    action        TEXT        NOT NULL,
    result        TEXT        NOT NULL,
    payload       JSONB       NOT NULL,
    payload_hash  TEXT        NOT NULL,
    previous_hash TEXT        NOT NULL,
    occurred_at   TIMESTAMPTZ NOT NULL
);

CREATE INDEX idx_audit_logs_tenant_time ON audit_logs (tenant_id, occurred_at DESC);
CREATE INDEX idx_audit_logs_actor_time ON audit_logs (actor_id, occurred_at DESC);

COMMENT ON TABLE  audit_logs               IS '审计日志';
COMMENT ON COLUMN audit_logs.seq           IS '链序(自增)';
COMMENT ON COLUMN audit_logs.log_id        IS '日志唯一ID';
COMMENT ON COLUMN audit_logs.actor_id      IS '操作者ID';
COMMENT ON COLUMN audit_logs.tenant_id     IS '租户ID';
COMMENT ON COLUMN audit_logs.action        IS '审计动作';
COMMENT ON COLUMN audit_logs.result        IS '操作结果';
COMMENT ON COLUMN audit_logs.payload       IS '完整日志内容(JSON)';
COMMENT ON COLUMN audit_logs.payload_hash  IS '内容SHA-256摘要';
COMMENT ON COLUMN audit_logs.previous_hash IS '上一条记录的链哈希';
COMMENT ON COLUMN audit_logs.occurred_at   IS '操作发生时间';

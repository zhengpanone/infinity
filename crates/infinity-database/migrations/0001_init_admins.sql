-- 初始化管理员表。
-- id / tenant_id 使用 TEXT，对齐 infinity-common 中 UserId / TenantId 的字符串 newtype。
CREATE TABLE IF NOT EXISTS admins (
    id         TEXT PRIMARY KEY,
    tenant_id  TEXT NOT NULL,
    username   TEXT NOT NULL UNIQUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

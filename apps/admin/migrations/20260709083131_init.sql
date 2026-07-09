-- Add migration script here

DROP TYPE IF EXISTS user_status_enum;
DO
$$
    BEGIN
        IF
            NOT EXISTS (SELECT 1 FROM pg_type where typname = 'user_status_enum') THEN
            CREATE TYPE user_status_enum as ENUM ('activate', 'deactivate', 'suspended', 'locked', 'pending', 'deleted');
        END IF;

    end
$$;


-- Create sys_user table
DROP TABLE IF EXISTS sys_user;

CREATE TABLE IF NOT EXISTS sys_user
(
    -- 主键
    id                   UUID PRIMARY KEY          DEFAULT gen_random_uuid(),
    
    -- 基本信息
    username             VARCHAR(50)      NOT NULL,
    email                VARCHAR(100)     NOT NULL UNIQUE,
    phone                VARCHAR(20),
    password_hash        VARCHAR(255)     NOT NULL,
    display_name         VARCHAR(100)     NOT NULL,
    avatar_url           VARCHAR(500),

    -- 验证状态
    email_verified       BOOLEAN          NOT NULL DEFAULT FALSE,
    phone_verified       BOOLEAN          NOT NULL DEFAULT FALSE,

    -- 用户状态
    status               user_status_enum NOT NULL DEFAULT 'pending',

    -- 登录相关
    last_login_at        TIMESTAMPTZ,
    login_count          INTEGER          NOT NULL DEFAULT 0,
    failed_login_count   INTEGER          NOT NULL DEFAULT 0,
    last_failed_login_at TIMESTAMPTZ,
    is_first_login       BOOLEAN          NOT NULL DEFAULT TRUE,
    last_activity_at     TIMESTAMPTZ,

    -- 账户安全
    locked_until         TIMESTAMPTZ,
    locked_at            TIMESTAMPTZ,
    lock_reason          VARCHAR(255),
    password_changed_at  TIMESTAMPTZ,
    password_expires_at  TIMESTAMPTZ,

    -- 时间戳
    created_at           TIMESTAMPTZ      NOT NULL DEFAULT NOW(),
    updated_at           TIMESTAMPTZ      NOT NULL DEFAULT NOW(),
    deleted_at           TIMESTAMPTZ
);

-- 添加注释
COMMENT ON TABLE sys_user IS '用户表';
COMMENT ON COLUMN sys_user.id IS '用户ID';
COMMENT ON COLUMN sys_user.username IS '用户名';
COMMENT ON COLUMN sys_user.email IS '邮箱';
COMMENT ON COLUMN sys_user.phone IS '手机号';
COMMENT ON COLUMN sys_user.password_hash IS '密码哈希';
COMMENT ON COLUMN sys_user.display_name IS '显示名称';
COMMENT ON COLUMN sys_user.avatar_url IS '头像URL';
COMMENT ON COLUMN sys_user.email_verified IS '邮箱是否已验证';
COMMENT ON COLUMN sys_user.phone_verified IS '手机号是否已验证';
COMMENT ON COLUMN sys_user.status IS '用户状态';
COMMENT ON COLUMN sys_user.last_login_at IS '最后登录时间';
COMMENT ON COLUMN sys_user.login_count IS '登录次数';
COMMENT ON COLUMN sys_user.failed_login_count IS '失败登录次数';
COMMENT ON COLUMN sys_user.last_failed_login_at IS '最后失败登录时间';
COMMENT ON COLUMN sys_user.locked_at IS '账户锁定时间';
COMMENT ON COLUMN sys_user.locked_until IS '锁定到期时间';
COMMENT ON COLUMN sys_user.lock_reason IS '账户锁定原因';
COMMENT ON COLUMN sys_user.password_changed_at IS '密码最后修改时间';
COMMENT ON COLUMN sys_user.password_expires_at IS '密码过期时间';
COMMENT ON COLUMN sys_user.is_first_login IS '是否首次登录';
COMMENT ON COLUMN sys_user.last_activity_at IS '上次活动时间';


COMMENT ON COLUMN sys_user.created_at IS '创建时间';
COMMENT ON COLUMN sys_user.updated_at IS '更新时间';
COMMENT ON COLUMN sys_user.deleted_at IS '软删除时间';

-- 创建索引
CREATE INDEX idx_sys_user_email ON sys_user (email);
CREATE INDEX idx_sys_user_username ON sys_user (username);
CREATE INDEX idx_sys_user_status ON sys_user (status);
CREATE INDEX idx_sys_user_deleted_at ON sys_user (deleted_at);



-- 创建 updated_at 触发器（如果不存在）
CREATE OR REPLACE FUNCTION trg_set_timestamp()
    RETURNS TRIGGER AS
$$
BEGIN
    NEW.updated_at := NOW();
    RETURN NEW;
END;
$$ language 'plpgsql';


DROP TRIGGER IF EXISTS set_sys_user_updated_at ON sys_user;

-- Create trigger for sys_user table
CREATE TRIGGER set_sys_user_updated_at
    BEFORE UPDATE
    ON sys_user
    FOR EACH ROW
EXECUTE FUNCTION trg_set_timestamp();



do
$$
    BEGIN
        IF
            NOT EXISTS (SELECT 1 FROM pg_type where typname = 'role_status_enum') THEN
            CREATE TYPE role_status_enum as ENUM ('active', 'inactive','banned');
        END IF;

    end
$$;
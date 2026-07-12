-- ============================================================
-- 初始迁移：核心账户、权限与系统配置模型
-- 包含表：
--   sys_user              用户表
--   sys_role              角色表
--   sys_user_role         用户-角色关联表
--   sys_config_category   配置分类（一级）
--   sys_config_group      配置分组（二级）
--   sys_config            配置项
-- 自定义类型：
--   user_status_enum      用户状态
--   role_status_enum      角色状态
--   role_type_enum        角色类型
--   config_type_enum      配置值类型
--   config_hint_enum      前端控件类型
-- 公共工具：
--   trg_set_timestamp()   自动维护 updated_at
-- ============================================================

-- ============================================================
-- 1. 用户相关
-- ============================================================

DROP TABLE IF EXISTS sys_user CASCADE;
DROP TABLE IF EXISTS sys_user_role CASCADE;
DROP TABLE IF EXISTS sys_role CASCADE;
DROP TYPE  IF EXISTS user_status_enum;

DO
$$
    BEGIN
        IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'user_status_enum') THEN
            CREATE TYPE user_status_enum AS ENUM
                ('activate', 'deactivate', 'suspended', 'locked', 'pending', 'deleted');
        END IF;
    END
$$;

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

    -- 审计字段
    created_id           VARCHAR(36)      NOT NULL DEFAULT '1',
    created_at           TIMESTAMPTZ      NOT NULL DEFAULT NOW(),
    created_by           VARCHAR(255)     NOT NULL DEFAULT 'system',
    updated_id           VARCHAR(36)      NOT NULL DEFAULT '1',
    updated_at           TIMESTAMPTZ      NOT NULL DEFAULT NOW(),
    updated_by           VARCHAR(255)     NOT NULL DEFAULT 'system',

    -- 软删除
    is_deleted           BOOLEAN          NOT NULL DEFAULT FALSE,
    deleted_at           TIMESTAMPTZ,

    CONSTRAINT chk_sys_user_email    CHECK (email ~* '^[^@\s]+@[^@\s]+\.[^@\s]+$' AND length(email) <= 254),
    CONSTRAINT chk_sys_user_username CHECK (length(username) BETWEEN 3 AND 50 AND username ~ '^[A-Za-z0-9_.-]+$'),
    CONSTRAINT chk_sys_user_phone    CHECK (phone IS NULL OR phone ~ '^\+?[0-9]{6,20}$'),
    CONSTRAINT chk_sys_user_pwd_hash CHECK (length(password_hash) >= 20),
    CONSTRAINT chk_sys_user_login    CHECK (login_count >= 0 AND failed_login_count >= 0),
    CONSTRAINT chk_sys_user_lock     CHECK (locked_at IS NULL OR locked_until IS NULL OR locked_until > locked_at)
);

COMMENT ON TABLE  sys_user                       IS '用户表';
COMMENT ON COLUMN sys_user.id                    IS '用户ID';
COMMENT ON COLUMN sys_user.username              IS '用户名';
COMMENT ON COLUMN sys_user.email                 IS '邮箱';
COMMENT ON COLUMN sys_user.phone                 IS '手机号';
COMMENT ON COLUMN sys_user.password_hash         IS '密码哈希';
COMMENT ON COLUMN sys_user.display_name          IS '显示名称';
COMMENT ON COLUMN sys_user.avatar_url            IS '头像URL';
COMMENT ON COLUMN sys_user.email_verified        IS '邮箱是否已验证';
COMMENT ON COLUMN sys_user.phone_verified        IS '手机号是否已验证';
COMMENT ON COLUMN sys_user.status                IS '用户状态';
COMMENT ON COLUMN sys_user.last_login_at         IS '最后登录时间';
COMMENT ON COLUMN sys_user.login_count           IS '登录次数';
COMMENT ON COLUMN sys_user.failed_login_count    IS '失败登录次数';
COMMENT ON COLUMN sys_user.last_failed_login_at  IS '最后失败登录时间';
COMMENT ON COLUMN sys_user.is_first_login        IS '是否首次登录';
COMMENT ON COLUMN sys_user.last_activity_at      IS '上次活动时间';
COMMENT ON COLUMN sys_user.locked_until          IS '锁定到期时间';
COMMENT ON COLUMN sys_user.locked_at             IS '账户锁定时间';
COMMENT ON COLUMN sys_user.lock_reason           IS '账户锁定原因';
COMMENT ON COLUMN sys_user.password_changed_at   IS '密码最后修改时间';
COMMENT ON COLUMN sys_user.password_expires_at   IS '密码过期时间';
COMMENT ON COLUMN sys_user.created_id            IS '创建人ID';
COMMENT ON COLUMN sys_user.created_at            IS '创建时间';
COMMENT ON COLUMN sys_user.created_by            IS '创建人';
COMMENT ON COLUMN sys_user.updated_id            IS '最后修改人ID';
COMMENT ON COLUMN sys_user.updated_at            IS '更新时间';
COMMENT ON COLUMN sys_user.updated_by            IS '最后修改人';
COMMENT ON COLUMN sys_user.is_deleted            IS '是否已删除';
COMMENT ON COLUMN sys_user.deleted_at            IS '软删除时间';

CREATE INDEX idx_sys_user_email       ON sys_user (email);
CREATE INDEX idx_sys_user_username    ON sys_user (username);
CREATE INDEX idx_sys_user_status      ON sys_user (status);
CREATE INDEX idx_sys_user_deleted     ON sys_user (is_deleted);
CREATE INDEX idx_sys_user_created_at  ON sys_user (created_at);
-- 后台管理分页常用：未删除 + 创建时间倒序
CREATE INDEX idx_sys_user_active_created
    ON sys_user (is_deleted, created_at DESC);
-- 按状态过滤常用：未删除 + 状态
CREATE INDEX idx_sys_user_active_status
    ON sys_user (is_deleted, status);

-- ============================================================
-- 2. 公共工具：updated_at 自动维护
-- ============================================================

CREATE OR REPLACE FUNCTION trg_set_timestamp()
    RETURNS TRIGGER AS
$$
BEGIN
    NEW.updated_at := NOW();
    RETURN NEW;
END;
$$ LANGUAGE 'plpgsql';

DROP TRIGGER IF EXISTS set_sys_user_updated_at ON sys_user;

CREATE TRIGGER set_sys_user_updated_at
    BEFORE UPDATE
    ON sys_user
    FOR EACH ROW
EXECUTE FUNCTION trg_set_timestamp();

-- ============================================================
-- 3. 角色相关
-- ============================================================

DROP TYPE IF EXISTS role_status_enum;
DROP TYPE IF EXISTS role_type_enum;

DO
$$
    BEGIN
        IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'role_status_enum') THEN
            CREATE TYPE role_status_enum AS ENUM ('active', 'inactive', 'banned');
        END IF;

        IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'role_type_enum') THEN
            CREATE TYPE role_type_enum AS ENUM ('system', 'business', 'custom');
        END IF;
    END
$$;

CREATE TABLE IF NOT EXISTS sys_role
(
    -- 主键
    id           UUID PRIMARY KEY   DEFAULT gen_random_uuid(),

    -- 基本信息
    role_code    VARCHAR(50)      NOT NULL,
    role_name    VARCHAR(50)      NOT NULL,
    role_desc    VARCHAR(255),
    remark       VARCHAR(255),

    -- 属性
    role_type    role_type_enum   NOT NULL DEFAULT 'system',
    role_status  role_status_enum NOT NULL DEFAULT 'active',
    order_num    INT              NOT NULL DEFAULT 1,
    is_default   BOOLEAN          NOT NULL DEFAULT FALSE,
    is_protected BOOLEAN          NOT NULL DEFAULT FALSE,

    -- 审计字段
    created_id    VARCHAR(36)      NOT NULL DEFAULT '1',
    created_at   TIMESTAMPTZ       NOT NULL DEFAULT NOW(),
    created_by    VARCHAR(255)     NOT NULL DEFAULT 'system',
    updated_id    VARCHAR(36)      NOT NULL DEFAULT '1',
    updated_at   TIMESTAMPTZ       NOT NULL DEFAULT NOW(),
    updated_by    VARCHAR(255)     NOT NULL DEFAULT 'system',

    -- 软删除
    is_deleted   BOOLEAN          NOT NULL DEFAULT FALSE,
    deleted_at   TIMESTAMPTZ
);

COMMENT ON TABLE  sys_role              IS '角色表';
COMMENT ON COLUMN sys_role.id           IS '角色ID';
COMMENT ON COLUMN sys_role.role_code    IS '角色编码，如 ADMIN, USER, SUPPORT';
COMMENT ON COLUMN sys_role.role_name    IS '角色名称';
COMMENT ON COLUMN sys_role.role_desc    IS '角色描述';
COMMENT ON COLUMN sys_role.remark       IS '备注';
COMMENT ON COLUMN sys_role.role_type    IS '角色类型：system-系统，business-业务，custom-自定义';
COMMENT ON COLUMN sys_role.role_status  IS '角色状态';
COMMENT ON COLUMN sys_role.order_num    IS '排序';
COMMENT ON COLUMN sys_role.is_default   IS '是否默认角色';
COMMENT ON COLUMN sys_role.is_protected IS '是否受保护角色（不允许删除）';
COMMENT ON COLUMN sys_role.created_id   IS '创建人ID';
COMMENT ON COLUMN sys_role.created_at   IS '创建时间';
COMMENT ON COLUMN sys_role.created_by   IS '创建人';
COMMENT ON COLUMN sys_role.updated_id   IS '最后修改人ID';
COMMENT ON COLUMN sys_role.updated_at   IS '更新时间';
COMMENT ON COLUMN sys_role.updated_by   IS '最后修改人';
COMMENT ON COLUMN sys_role.is_deleted   IS '是否已删除';
COMMENT ON COLUMN sys_role.deleted_at   IS '软删除时间';

CREATE INDEX        idx_sys_role_code     ON sys_role (role_code);
CREATE INDEX        idx_sys_role_status   ON sys_role (role_status);
CREATE INDEX        idx_sys_role_deleted  ON sys_role (is_deleted);
CREATE INDEX        idx_sys_role_active_status ON sys_role (is_deleted, role_status);
CREATE UNIQUE INDEX uk_sys_role_code_active ON sys_role (role_code)  WHERE is_deleted = FALSE;

DROP TRIGGER IF EXISTS set_sys_role_updated_at ON sys_role;

CREATE TRIGGER set_sys_role_updated_at
    BEFORE UPDATE
    ON sys_role
    FOR EACH ROW
EXECUTE FUNCTION trg_set_timestamp();

-- ============================================================
-- 4. 用户-角色关联
-- ============================================================

CREATE TABLE IF NOT EXISTS sys_user_role
(
    -- 主键
    id             UUID PRIMARY KEY DEFAULT gen_random_uuid(),

    -- 关联
    user_id        UUID         NOT NULL,
    role_id        UUID         NOT NULL,

    -- 来源与有效期
    source         INT          NOT NULL DEFAULT 1,
    effective_from TIMESTAMPTZ,
    effective_to   TIMESTAMPTZ,

    -- 审计字段
    created_id     VARCHAR(36)  NOT NULL DEFAULT '1',
    created_at     TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    created_by     VARCHAR(255) NOT NULL DEFAULT 'system',
    updated_id     VARCHAR(36)  NOT NULL DEFAULT '1',
    updated_at     TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    updated_by     VARCHAR(255) NOT NULL DEFAULT 'system',

    -- 软删除
    is_deleted     BOOLEAN      NOT NULL DEFAULT FALSE,
    deleted_at     TIMESTAMPTZ,

    CONSTRAINT chk_user_role_effective CHECK (effective_from IS NULL OR effective_to IS NULL OR effective_from < effective_to)
);

COMMENT ON TABLE  sys_user_role                IS '用户-角色关联表';
COMMENT ON COLUMN sys_user_role.id             IS '关联ID';
COMMENT ON COLUMN sys_user_role.user_id        IS '用户ID';
COMMENT ON COLUMN sys_user_role.role_id        IS '角色ID';
COMMENT ON COLUMN sys_user_role.source         IS '来源：1-手动分配，2-自动分配，3-继承';
COMMENT ON COLUMN sys_user_role.effective_from IS '生效时间';
COMMENT ON COLUMN sys_user_role.effective_to   IS '失效时间';
COMMENT ON COLUMN sys_user_role.created_id     IS '创建人ID';
COMMENT ON COLUMN sys_user_role.created_at     IS '创建时间';
COMMENT ON COLUMN sys_user_role.created_by     IS '创建人';
COMMENT ON COLUMN sys_user_role.updated_id     IS '最后修改人ID';
COMMENT ON COLUMN sys_user_role.updated_at     IS '更新时间';
COMMENT ON COLUMN sys_user_role.updated_by     IS '最后修改人';
COMMENT ON COLUMN sys_user_role.is_deleted     IS '是否已删除';
COMMENT ON COLUMN sys_user_role.deleted_at     IS '软删除时间';

CREATE INDEX idx_user_role_user          ON sys_user_role (user_id);
CREATE INDEX idx_user_role_role          ON sys_user_role (role_id);
CREATE INDEX idx_user_role_deleted       ON sys_user_role (is_deleted);
CREATE INDEX idx_user_role_active_user   ON sys_user_role (user_id, is_deleted);
CREATE UNIQUE INDEX uk_user_role_active  ON sys_user_role (user_id, role_id) WHERE is_deleted = FALSE;

DROP TRIGGER IF EXISTS set_sys_user_role_updated_at ON sys_user_role;

CREATE TRIGGER set_sys_user_role_updated_at
    BEFORE UPDATE
    ON sys_user_role
    FOR EACH ROW
EXECUTE FUNCTION trg_set_timestamp();

-- ============================================================
-- 5. 配置相关
-- ============================================================

DROP TABLE IF EXISTS sys_config CASCADE;
DROP TABLE IF EXISTS sys_config_group CASCADE;
DROP TABLE IF EXISTS sys_config_category CASCADE;
DROP TYPE  IF EXISTS config_type_enum;
DROP TYPE  IF EXISTS config_hint_enum;

DO
$$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'config_type_enum') THEN
        CREATE TYPE config_type_enum AS ENUM
            ('string', 'number', 'boolean', 'json', 'password',
             'url', 'email', 'duration', 'cron', 'color');
    END IF;

    IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'config_hint_enum') THEN
        CREATE TYPE config_hint_enum AS ENUM
            ('text', 'textarea', 'number', 'switch', 'password',
             'json', 'select', 'radio', 'checkbox', 'cron', 'color');
    END IF;
END
$$;

CREATE TABLE IF NOT EXISTS sys_config_category
(
    id              VARCHAR(36) PRIMARY KEY DEFAULT gen_random_uuid(),

    category_code   VARCHAR(100) NOT NULL,
    category_name   VARCHAR(100) NOT NULL,
    icon            VARCHAR(100),
    color           VARCHAR(50),
    order_num       INT          NOT NULL DEFAULT 1,
    remark          TEXT,
    category_desc   TEXT,
    is_builtin      BOOLEAN      NOT NULL DEFAULT FALSE,

    created_id      VARCHAR(36)  NOT NULL DEFAULT '1',
    created_at      TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    created_by      VARCHAR(255) NOT NULL DEFAULT 'system',

    updated_id      VARCHAR(36)  NOT NULL DEFAULT '1',
    updated_at      TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    updated_by      VARCHAR(255) NOT NULL DEFAULT 'system',

    is_deleted      BOOLEAN      NOT NULL DEFAULT FALSE,
    deleted_at      TIMESTAMPTZ
);

COMMENT ON TABLE  sys_config_category                  IS '系统配置-一级分类';
COMMENT ON COLUMN sys_config_category.id              IS '分类ID';
COMMENT ON COLUMN sys_config_category.category_code   IS '分类编码';
COMMENT ON COLUMN sys_config_category.category_name   IS '分类名称';
COMMENT ON COLUMN sys_config_category.icon            IS '分类图标';
COMMENT ON COLUMN sys_config_category.color           IS '分类主题色';
COMMENT ON COLUMN sys_config_category.order_num       IS '排序';
COMMENT ON COLUMN sys_config_category.remark          IS '备注';
COMMENT ON COLUMN sys_config_category.category_desc   IS '分类描述';
COMMENT ON COLUMN sys_config_category.is_builtin      IS '是否系统内置';
COMMENT ON COLUMN sys_config_category.created_id      IS '创建人ID';
COMMENT ON COLUMN sys_config_category.created_at      IS '创建时间';
COMMENT ON COLUMN sys_config_category.created_by      IS '创建人';
COMMENT ON COLUMN sys_config_category.updated_id      IS '最后修改人ID';
COMMENT ON COLUMN sys_config_category.updated_at      IS '更新时间';
COMMENT ON COLUMN sys_config_category.updated_by      IS '最后修改人';
COMMENT ON COLUMN sys_config_category.is_deleted      IS '是否已删除';
COMMENT ON COLUMN sys_config_category.deleted_at      IS '软删除时间';

DROP TRIGGER IF EXISTS set_sys_config_category_updated_at ON sys_config_category;

CREATE TRIGGER set_sys_config_category_updated_at
    BEFORE UPDATE
    ON sys_config_category
    FOR EACH ROW
EXECUTE FUNCTION trg_set_timestamp();

CREATE UNIQUE INDEX uk_sys_config_category_code_active
    ON sys_config_category (category_code)
    WHERE is_deleted = FALSE;

CREATE INDEX idx_config_category_deleted
    ON sys_config_category (is_deleted);

CREATE TABLE IF NOT EXISTS sys_config_group
(
    id              VARCHAR(36) PRIMARY KEY DEFAULT gen_random_uuid(),

    category_code   VARCHAR(100) NOT NULL,
    group_code      VARCHAR(100) NOT NULL,
    group_name      VARCHAR(100) NOT NULL,
    icon            VARCHAR(100),
    order_num       INT          NOT NULL DEFAULT 1,
    remark          TEXT,
    group_desc      TEXT,
    is_builtin      BOOLEAN      NOT NULL DEFAULT FALSE,

    created_id      VARCHAR(36)  NOT NULL DEFAULT '1',
    created_at      TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    created_by      VARCHAR(255) NOT NULL DEFAULT 'system',

    updated_id      VARCHAR(36)  NOT NULL DEFAULT '1',
    updated_at      TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    updated_by      VARCHAR(255) NOT NULL DEFAULT 'system',

    is_deleted      BOOLEAN      NOT NULL DEFAULT FALSE,
    deleted_at      TIMESTAMPTZ
);
COMMENT ON TABLE  sys_config_group                  IS '系统配置-二级分组';
COMMENT ON COLUMN sys_config_group.id              IS '分组ID';
COMMENT ON COLUMN sys_config_group.category_code   IS '一级分类编码';
COMMENT ON COLUMN sys_config_group.group_code      IS '分组编码';
COMMENT ON COLUMN sys_config_group.group_name      IS '分组名称';
COMMENT ON COLUMN sys_config_group.icon            IS '分组图标';
COMMENT ON COLUMN sys_config_group.order_num       IS '排序';
COMMENT ON COLUMN sys_config_group.remark          IS '备注';
COMMENT ON COLUMN sys_config_group.group_desc      IS '分组描述';
COMMENT ON COLUMN sys_config_group.is_builtin      IS '是否系统内置';
COMMENT ON COLUMN sys_config_group.created_id      IS '创建人ID';
COMMENT ON COLUMN sys_config_group.created_at      IS '创建时间';
COMMENT ON COLUMN sys_config_group.created_by      IS '创建人';
COMMENT ON COLUMN sys_config_group.updated_id      IS '最后修改人ID';
COMMENT ON COLUMN sys_config_group.updated_at      IS '更新时间';
COMMENT ON COLUMN sys_config_group.updated_by      IS '最后修改人';
COMMENT ON COLUMN sys_config_group.is_deleted      IS '是否已删除';
COMMENT ON COLUMN sys_config_group.deleted_at      IS '软删除时间';

CREATE INDEX        idx_config_group_category       ON sys_config_group (category_code);
CREATE INDEX        idx_config_group_deleted        ON sys_config_group (is_deleted);
CREATE UNIQUE INDEX uk_config_group_active
    ON sys_config_group (category_code, group_code)
    WHERE is_deleted = FALSE;

DROP TRIGGER IF EXISTS set_sys_config_group_updated_at ON sys_config_group;

CREATE TRIGGER set_sys_config_group_updated_at
    BEFORE UPDATE
    ON sys_config_group
    FOR EACH ROW
EXECUTE FUNCTION trg_set_timestamp();

CREATE TABLE IF NOT EXISTS sys_config
(
    id              VARCHAR(36) PRIMARY KEY DEFAULT gen_random_uuid(),

    category_code   VARCHAR(100) NOT NULL,
    group_code      VARCHAR(100) NOT NULL,

    config_key      VARCHAR(255) NOT NULL,
    config_name     VARCHAR(255) NOT NULL,
    config_value    TEXT,
    default_value   TEXT,

    config_type     config_type_enum NOT NULL DEFAULT 'string',
    value_hint      config_hint_enum NOT NULL DEFAULT 'text',
    value_unit      VARCHAR(50),

    validation_rule TEXT,
    options         JSONB NOT NULL DEFAULT '[]'::jsonb
                    CHECK (jsonb_typeof(options) = 'array'),

    is_visible      BOOLEAN NOT NULL DEFAULT TRUE,
    is_editable     BOOLEAN NOT NULL DEFAULT TRUE,
    is_builtin      BOOLEAN NOT NULL DEFAULT FALSE,
    is_encrypted    BOOLEAN NOT NULL DEFAULT FALSE,
    version         BIGINT  NOT NULL DEFAULT 0 CHECK (version >= 0),

    order_num       INT     NOT NULL DEFAULT 1,
    remark          TEXT,

    created_id      VARCHAR(36)  NOT NULL DEFAULT '1',
    created_at      TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    created_by      VARCHAR(255) NOT NULL DEFAULT 'system',

    updated_id      VARCHAR(36)  NOT NULL DEFAULT '1',
    updated_at      TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    updated_by      VARCHAR(255) NOT NULL DEFAULT 'system',

    is_deleted      BOOLEAN      NOT NULL DEFAULT FALSE,
    deleted_at      TIMESTAMPTZ
);

COMMENT ON TABLE  sys_config               IS '系统配置表';
COMMENT ON COLUMN sys_config.id           IS '配置ID';
COMMENT ON COLUMN sys_config.category_code IS '一级分类编码';
COMMENT ON COLUMN sys_config.group_code    IS '二级分组编码';
COMMENT ON COLUMN sys_config.config_key    IS '配置Key，全局唯一';
COMMENT ON COLUMN sys_config.config_name   IS '配置名称';
COMMENT ON COLUMN sys_config.config_value  IS '配置值';
COMMENT ON COLUMN sys_config.default_value IS '默认值';
COMMENT ON COLUMN sys_config.config_type   IS '配置值类型';
COMMENT ON COLUMN sys_config.value_hint    IS '前端控件类型';
COMMENT ON COLUMN sys_config.value_unit    IS '值单位';
COMMENT ON COLUMN sys_config.validation_rule IS '校验规则（正则或表达式）';
COMMENT ON COLUMN sys_config.options       IS '枚举选项（JSON数组）';
COMMENT ON COLUMN sys_config.is_visible    IS '是否在后台显示';
COMMENT ON COLUMN sys_config.is_editable   IS '是否允许修改';
COMMENT ON COLUMN sys_config.is_builtin    IS '是否系统内置';
COMMENT ON COLUMN sys_config.is_encrypted  IS '是否加密存储';
COMMENT ON COLUMN sys_config.version       IS '配置版本号';
COMMENT ON COLUMN sys_config.order_num     IS '排序';
COMMENT ON COLUMN sys_config.remark        IS '备注';
COMMENT ON COLUMN sys_config.created_id    IS '创建人ID';
COMMENT ON COLUMN sys_config.created_at    IS '创建时间';
COMMENT ON COLUMN sys_config.created_by    IS '创建人';
COMMENT ON COLUMN sys_config.updated_id    IS '最后修改人ID';
COMMENT ON COLUMN sys_config.updated_at    IS '更新时间';
COMMENT ON COLUMN sys_config.updated_by    IS '最后修改人';
COMMENT ON COLUMN sys_config.is_deleted    IS '是否已删除';
COMMENT ON COLUMN sys_config.deleted_at    IS '软删除时间';

CREATE INDEX        idx_sys_config_category        ON sys_config (category_code);
CREATE INDEX        idx_sys_config_group           ON sys_config (group_code);
CREATE INDEX        idx_sys_config_category_group  ON sys_config (category_code, group_code);
CREATE INDEX        idx_sys_config_deleted         ON sys_config (is_deleted);
CREATE INDEX        idx_sys_config_category_deleted ON sys_config (category_code, is_deleted);
CREATE INDEX        idx_sys_config_group_deleted   ON sys_config (group_code, is_deleted);
CREATE INDEX        idx_sys_config_options_gin     ON sys_config USING GIN (options);
CREATE UNIQUE INDEX uk_sys_config_key_active
    ON sys_config (config_key)
    WHERE is_deleted = FALSE;

DROP TRIGGER IF EXISTS set_sys_config_updated_at ON sys_config;

CREATE TRIGGER set_sys_config_updated_at
    BEFORE UPDATE
    ON sys_config
    FOR EACH ROW
EXECUTE FUNCTION trg_set_timestamp();


CREATE TABLE sys_dict_type
(
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),

    dict_code       VARCHAR(100) NOT NULL,
    dict_name       VARCHAR(100) NOT NULL,

    icon            VARCHAR(100),
    color           VARCHAR(50),

    description     TEXT,

    is_builtin      BOOLEAN NOT NULL DEFAULT FALSE,

    is_enabled      BOOLEAN NOT NULL DEFAULT TRUE,

    order_num       INT NOT NULL DEFAULT 1,

    remark          TEXT,

    created_id      VARCHAR(36) NOT NULL DEFAULT '1',
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_by      VARCHAR(255) NOT NULL DEFAULT 'system',

    updated_id      VARCHAR(36) NOT NULL DEFAULT '1',
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_by      VARCHAR(255) NOT NULL DEFAULT 'system',

    is_deleted      BOOLEAN NOT NULL DEFAULT FALSE,
    deleted_at      TIMESTAMPTZ
);

CREATE UNIQUE INDEX uk_dict_type
    ON sys_dict_type(dict_code)
    WHERE is_deleted = FALSE;

CREATE TABLE sys_dict_item
(
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),

    dict_code       VARCHAR(100) NOT NULL,

    item_code       VARCHAR(100) NOT NULL,

    item_label      VARCHAR(200) NOT NULL,

    item_value      VARCHAR(500) NOT NULL,

    item_color      VARCHAR(50),

    icon            VARCHAR(100),

    css_class       VARCHAR(100),

    ext_data        JSONB NOT NULL DEFAULT '{}'::jsonb,

    is_default      BOOLEAN NOT NULL DEFAULT FALSE,

    is_enabled      BOOLEAN NOT NULL DEFAULT TRUE,

    order_num       INT NOT NULL DEFAULT 1,

    remark          TEXT,

    created_id      VARCHAR(36) NOT NULL DEFAULT '1',
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_by      VARCHAR(255) NOT NULL DEFAULT 'system',

    updated_id      VARCHAR(36) NOT NULL DEFAULT '1',
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_by      VARCHAR(255) NOT NULL DEFAULT 'system',

    is_deleted      BOOLEAN NOT NULL DEFAULT FALSE,
    deleted_at      TIMESTAMPTZ
);
CREATE UNIQUE INDEX uk_dict_item
    ON sys_dict_item(dict_code,item_code)
    WHERE is_deleted=FALSE;

CREATE TABLE sys_dict_item_i18n
(
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),

    item_id UUID NOT NULL,

    locale VARCHAR(20) NOT NULL,

    label VARCHAR(255) NOT NULL,

    description TEXT
);
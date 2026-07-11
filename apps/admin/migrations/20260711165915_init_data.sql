-- ============================================================
-- 初始数据：系统配置默认项
-- 依赖表：sys_config_category / sys_config_group / sys_config
-- 说明：
--   - 分类 category_code 必须在 sys_config_category 中存在
--   - 分组 (category_code, group_code) 必须在 sys_config_group 中存在
--   - 所有数据为系统内置 (is_builtin = TRUE)
-- ============================================================

INSERT INTO sys_config_category (id, category_code, category_name, is_builtin, order_num)
VALUES
    ('00000000-0000-0000-0000-000000000001', 'system', '系统配置',   TRUE, 1),
    ('00000000-0000-0000-0000-000000000002', 'login',  '登录配置',   TRUE, 2),
    ('00000000-0000-0000-0000-000000000003', 'upload', '上传配置',   TRUE, 3),
    ('00000000-0000-0000-0000-000000000004', 'ai',     'AI 配置',    TRUE, 4)
ON CONFLICT (category_code) WHERE is_deleted = FALSE DO NOTHING;

INSERT INTO sys_config_group (id, category_code, group_code, group_name, is_builtin, order_num)
VALUES
    ('00000000-0000-0000-0000-000000000101', 'system', 'base',  '基础配置',   TRUE, 1),
    ('00000000-0000-0000-0000-000000000201', 'login',  'base',  '基础配置',   TRUE, 1),
    ('00000000-0000-0000-0000-000000000301', 'upload', 'base',  '基础配置',   TRUE, 1),
    ('00000000-0000-0000-0000-000000000401', 'ai',     'base',  '基础配置',   TRUE, 1)
ON CONFLICT (category_code, group_code) WHERE is_deleted = FALSE DO NOTHING;

INSERT INTO sys_config (
    id, category_code, group_code, config_key, config_name,
    config_value, config_type, value_hint, is_builtin
)
VALUES
    ('00000000-0000-0000-0000-000000001001', 'system', 'base', 'system.name',         '系统名称',        'Infinity',                 'string',  'text',     TRUE),
    ('00000000-0000-0000-0000-000000001002', 'system', 'base', 'system.version',      '系统版本',        '1.0.0',                    'string',  'text',     TRUE),
    ('00000000-0000-0000-0000-000000002001', 'login',  'base', 'login.max_retry',     '登录失败次数',    '5',                        'number',  'number',   TRUE),
    ('00000000-0000-0000-0000-000000002002', 'login',  'base', 'login.lock_minutes',  '锁定时间',        '30',                       'number',  'number',   TRUE),
    ('00000000-0000-0000-0000-000000003001', 'upload', 'base', 'upload.max_size',     '上传大小(MB)',    '50',                       'number',  'number',   TRUE),
    ('00000000-0000-0000-0000-000000003002', 'upload', 'base', 'upload.allowed_suffix','允许上传类型',    'jpg,png,gif,pdf,docx,xlsx', 'string',  'textarea', TRUE),
    ('00000000-0000-0000-0000-000000004001', 'ai',     'base', 'ai.enable',           '启用 AI',         'true',                     'boolean', 'switch',   TRUE),
    ('00000000-0000-0000-0000-000000004002', 'ai',     'base', 'ai.provider',         'AI 服务商',       'openai',                   'string',  'select',   TRUE)
ON CONFLICT (config_key) WHERE is_deleted = FALSE DO NOTHING;
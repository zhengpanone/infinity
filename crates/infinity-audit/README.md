# infinity-audit 审计日志

记录"谁在什么时候对什么做了什么、结果如何"。

## 模块结构

| 模块 | 内容 |
| --- | --- |
| `domain` | `AuditAction` 动作、`AuditResult` 结果、`AuditPolicy` 落库策略、`AuditLog` 实体、`AuditRecord` 防篡改哈希链、`FieldChange` 字段级变更 |
| `mask` | 手机号 / 密码脱敏、机密字段判定 `is_secret_field` |
| `repository` | `AuditRepository` 抽象,`PgAuditRepository`(PostgreSQL)与 `InMemoryAuditRepository`(测试)实现 |
| `service` | `AuditService`:高危操作 `MustRecord`(失败上抛),普通操作 `BestEffort`(失败仅告警) |
| `middleware` | axum 中间件:handler 插入 `AuditContext`,中间件采集 IP / UA / 请求 ID 并按 HTTP 状态推断结果落库 |

## 使用

```rust
// 1. 组装服务
let repo = Arc::new(PgAuditRepository::new(pool));
let audit = AuditService::new(repo);

// 2. 挂中间件
let app = Router::new()
    .route("/users/{id}", delete(delete_user))
    .layer(middleware::from_fn_with_state(audit.clone(), audit_middleware));

// 3. handler 内声明审计上下文
let ctx = AuditContext::new(actor_id, tenant_id, AuditAction::Delete)
    .target(user_id, username, "user")
    .description("删除用户");
response.extensions_mut().insert(ctx);
```

`audit_logs` 建表 SQL 见 `PgAuditRepository` 的文档注释。

## 已实现的合规能力

- 高危操作(授权变更、退款、导出)强制落库,失败则业务请求失败;
- 高危操作被拒绝时输出 error 级告警日志;
- SHA-256 哈希链防篡改,`AuditRecord::verify_chain` 可离线校验;
- 敏感字段(密码、手机号)落库前脱敏。

## 后续增加

| 能力      | 方案             |
| ------- | -------------- |
| 海量日志    | PostgreSQL 分区表 |
| 半年以上归档  | ClickHouse     |
| 实时审计    | Kafka/Pulsar   |
| 检索      | Elasticsearch  |
| 安全合规    | WORM存储         |
| 管理员查看日志 | RBAC + 二次审计    |
| 高危操作    | 审批流            |

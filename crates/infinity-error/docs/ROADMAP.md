# Infinity Error 路线图

## Milestone 1：共享核心

- [x] 添加 `InfinityError`
- [x] 添加 `Result<T>`
- [x] 添加 `ErrorKind`
- [x] 添加基础设施和 API 场景的构造函数
- [x] 添加默认状态码映射
- [x] 添加单元测试
- [x] 添加 README 和设计文档
- [x] 为 `ErrorKind` / `InfinityError` 添加 `#[non_exhaustive]`
- [x] 添加 `InfinityError::with_kind` 通用构造函数
- [x] 添加 `ResultExt` 边界转换扩展 trait
- [x] `ResultExt` / `with_source` 保留类型化来源错误链
- [x] 添加 `ErrorKind::ALL` 与 `ErrorKind::from_code` 反查
- [x] 添加 `bail!` / `ensure!` 宏

## Milestone 2：工作区接入

- [x] 为 `infinity-config` 添加 `From<ConfigError> for InfinityError` 边界转换
- [x] 为 `infinity-logger` 添加 `From<LoggerError> for InfinityError` 边界转换
- [x] 在 `apps/admin` 启动代码中用 `infinity_error::Result` 替换 `Box<dyn Error>`
- [ ] `infinity-database` 具备实际功能后接入 `infinity_error`（当前为占位桩）
- [ ] `infinity-cache` 具备实际功能后接入 `infinity_error`（当前为占位桩）
- [ ] `infinity-web` 具备实际功能后接入 `infinity_error`（当前为占位桩）

> 说明：`infinity-config` 与 `infinity-logger` 保留各自的富错误类型（`ConfigError` /
> `LoggerError`），仅在工作区边界通过 `From` 转换为 `InfinityError`，符合设计文档
> 「crate 可保留本地错误类型，在边界处转换」的原则。

## Milestone 3：Web 集成

- [x] 在 `infinity-web` 中添加响应映射（`ApiError` + `IntoResponse`）
- [x] 定义 API 错误响应结构（`status` / `code` / `message` / `request_id`）
- [x] 添加带 request-id 的错误日志建议（见 infinity-web README）

## Milestone 4：可观测性

- [x] 添加结构化错误日志规范
- [x] 记录 trace/span 中建议携带的错误字段
- [x] 基于 `ErrorKind::code()` 定义 metrics label

> 说明：新增 `ErrorClass`（`client` / `server`）、`InfinityError::class()`、
> `chain()` / `chain_string()` / `root_cause()` 与 `field` 字段键常量模块，均为零依赖
> 原语；具体记录动作在持有 `tracing` / `metrics` 依赖的边界处进行。规范见
> [OBSERVABILITY.md](OBSERVABILITY.md)。

## Milestone 5：API 冻结

- [ ] 在 database/cache/web/auth 接入后复查所有变体
- [ ] 冻结稳定的 `ErrorKind` 错误码
- [ ] 为下游 crate 添加迁移说明

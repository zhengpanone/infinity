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

- [ ] 在 `infinity-database` 中使用 `infinity_error::Result`
- [ ] 在 `infinity-cache` 中使用 `infinity_error::Result`
- [ ] 在 `infinity-web` 中使用 `infinity_error::Result`
- [ ] 在合适的位置替换应用启动代码中的 `Box<dyn Error>`

## Milestone 3：Web 集成

- [ ] 在 `infinity-web` 中添加响应映射
- [ ] 定义 API 错误响应结构
- [ ] 添加带 request-id 的错误日志建议

## Milestone 4：可观测性

- [ ] 添加结构化错误日志规范
- [ ] 记录 trace/span 中建议携带的错误字段
- [ ] 基于 `ErrorKind::code()` 定义 metrics label

## Milestone 5：API 冻结

- [ ] 在 database/cache/web/auth 接入后复查所有变体
- [ ] 冻结稳定的 `ErrorKind` 错误码
- [ ] 为下游 crate 添加迁移说明

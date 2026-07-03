# Request for Comments (RFC)

> **Project:** infinity-logger  
> **Version:** v1.0  
> **Status:** Active

本文档记录 Infinity Logger 的重要设计决策和提案。

---

## RFC 流程

### 何时需要 RFC？

以下情况需要编写 RFC：

- 添加/修改公开 API
- 重大架构变更
- 影响性能的变更
- 引入新依赖
- Breaking Changes

### RFC 格式

```markdown
# RFC-NNNN: Title

- **状态**: Draft / Review / Accepted / Rejected
- **作者**: @username
- **日期**: YYYY-MM-DD

## 摘要

一段话描述提案。

## 动机

为什么需要这个变更？

## 详细设计

如何实现？

## 缺点

有什么缺点？

## 备选方案

还有其他方案吗？

## 未解决问题

还有什么需要讨论？
```

---

## 已接受的 RFC

### RFC-0001: Builder Pattern

**状态**: Accepted  
**日期**: 2026-06-01

采用 Builder 模式作为主要 API 设计。

**动机**: 
- 灵活配置
- 类型安全
- 链式调用

**决策**: 
```rust
Logger::builder()
    .level(LogLevel::Info)
    .init()?;
```

详见: [ADR-0001](ADR/0001-builder.md)

---

### RFC-0002: Tracing Ecosystem

**状态**: Accepted  
**日期**: 2026-06-05

基于 `tracing` 而非 `log`。

**动机**:
- 结构化日志
- 异步支持
- 更好的性能
- 活跃的生态

**决策**: 依赖 `tracing` + `tracing-subscriber`

详见: [ADR-0002](ADR/0002-tracing.md)

---

### RFC-0003: Configuration Driven

**状态**: Accepted  
**日期**: 2026-06-10

所有行为通过配置驱动。

**动机**:
- 可预测
- 可测试
- 易维护

**决策**: Builder → Config → Init

详见: [ADR-0003](ADR/0003-config.md)

---

### RFC-0004: JSON Formatter

**状态**: Accepted  
**日期**: 2026-06-15

支持 JSON 格式输出。

**动机**:
- 结构化日志
- 便于日志系统解析
- 生产环境标配

**决策**: 通过 feature gate 启用

详见: [ADR-0004](ADR/0004-json.md)

---

## 待讨论的 RFC

### RFC-0005: Dynamic Reload

**状态**: Draft  
**作者**: @maintainer  
**日期**: 2026-07-01

**摘要**: 支持运行时动态调整日志级别。

**动机**:
- 生产环境调试
- 无需重启服务
- 临时提高日志级别

**设计**:
```rust
Logger::reload(LogLevel::Debug)?;
```

**实现**:
- 使用 `tracing-subscriber::reload`
- 保存 reload handle
- 提供全局 API

**缺点**:
- 增加复杂度
- 性能轻微下降（reload handle 开销）

**备选方案**:
1. 重启服务
2. 信号量控制（UNIX only）
3. 配置中心集成

**状态**: 需要社区反馈

---

### RFC-0006: Sampling

**状态**: Draft  
**作者**: TBD  
**日期**: TBD

**摘要**: 高负载场景下采样记录日志。

**动机**:
- 降低日志量
- 减少性能影响
- 保留代表性日志

**设计**:
```rust
Logger::builder()
    .sampling(SamplingConfig {
        rate: 0.1,  // 10% 采样率
        burst: 100, // 前 100 条全记录
    })
    .init()?;
```

**状态**: 收集需求中

---

## 已拒绝的 RFC

### RFC-XXXX: Custom Log Macro

**状态**: Rejected  
**日期**: 2026-06-20  
**原因**: 与 `tracing` 宏冲突，增加学习成本

**原提案**: 提供自定义日志宏
```rust
log_info!("message");
```

**拒绝理由**:
- 与 `tracing::info!` 功能重复
- 增加维护负担
- 用户需要学习新宏

**替代方案**: 直接使用 `tracing` 宏

---

## 提交 RFC

### 流程

1. 复制模板
2. 填写提案
3. 提交 PR 到 `docs/RFC/`
4. 讨论和修改
5. 投票决定

### 模板

```markdown
# RFC-NNNN: Title

- **状态**: Draft
- **作者**: @username
- **日期**: YYYY-MM-DD

## 摘要

## 动机

## 详细设计

## 缺点

## 备选方案

## 未解决问题
```

---

## 参考

- [Rust RFC Process](https://github.com/rust-lang/rfcs)
- [ADR (Architecture Decision Records)](ADR/)

---

**欢迎提交 RFC！**

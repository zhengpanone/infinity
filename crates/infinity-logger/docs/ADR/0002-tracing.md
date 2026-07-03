# ADR-0002: Tracing Ecosystem

**状态**: Accepted  
**日期**: 2026-06-05  
**作者**: @maintainer

---

## 背景

需要选择日志框架基础。

可选方案：
1. `log` crate
2. `tracing` crate
3. `slog` crate
4. 自研

---

## 决策

采用 **tracing 生态**。

### 核心依赖

```toml
[dependencies]
tracing = "0.1"
tracing-core = "0.1"
tracing-subscriber = "0.3"
tracing-appender = "0.2"
```

---

## 理由

### Tracing 优势

1. **结构化日志**
   ```rust
   tracing::info!(user_id = 123, "Login success");
   ```

2. **异步友好**
   - 与 Tokio 深度集成
   - 零成本抽象

3. **Span 支持**
   ```rust
   let _span = info_span!("request").entered();
   ```

4. **性能优秀**
   - 延迟计算
   - 条件编译优化

5. **活跃生态**
   - Tokio 官方维护
   - 生态完善

6. **OpenTelemetry 集成**
   - `tracing-opentelemetry`
   - 原生支持链路追踪

---

## 备选方案

### 方案 1: log crate

**优点**:
- 简单
- 历史悠久
- 兼容性好

**缺点**:
- 仅支持文本日志
- 无结构化字段
- 无 Span 概念
- 性能较差

### 方案 2: slog

**优点**:
- 结构化日志
- 性能好

**缺点**:
- 与 Tokio 集成差
- 生态较小
- API 复杂

### 方案 3: 自研

**缺点**:
- 开发成本高
- 维护负担重
- 生态缺失

---

## 实现细节

### 架构

```
infinity-logger (门面层)
        ↓
tracing-subscriber (组装层)
        ↓
tracing (事件层)
```

### 核心模块

- `tracing` - 日志宏和事件
- `tracing-subscriber` - Subscriber 实现
- `tracing-appender` - 文件输出
- `tracing-log` - log crate 桥接

---

## 兼容性

### 与 log crate 兼容

通过 `tracing-log` 自动桥接：

```rust
// 第三方库使用 log
log::info!("Message");

// 自动转发到 tracing
// ✅ 无需改动
```

---

## 影响

### 用户代码

用户使用 `tracing` 宏：

```rust
use tracing::{info, debug, error};

info!("Application started");
debug!(count = items.len(), "Processing");
error!("Failed");
```

### 性能

- **开销**: < 50ns per event（过滤后）
- **内存**: 最小化分配
- **异步**: 非阻塞写入

---

## 风险

### 依赖风险

**问题**: 依赖 Tokio 团队维护的 crate

**缓解**:
- Tokio 生态稳定
- 社区活跃
- 有后备方案（可切换到 log）

---

## 相关

- [tracing 文档](https://docs.rs/tracing)
- [Tokio 博客](https://tokio.rs/blog)
- [RFC-0002](../RFC.md#rfc-0002-tracing-ecosystem)

---

## 修订历史

| 日期       | 变更           |
|-----------|----------------|
| 2026-06-05 | 初始决策       |

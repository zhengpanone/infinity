# Infinity Error 可观测性规范

本文规定 Infinity 工作区在**日志、trace/span、metrics** 三个维度上如何记录
`InfinityError`，使各 crate 输出的字段名与取值一致，从而能够跨服务聚合、检索与告警。

> `infinity-error` 有意保持低依赖（仅 `thiserror`），本身不依赖 `tracing` /
> `metrics`。本规范提供**零依赖的字段常量与分类类型**，具体记录动作发生在已经持有
> 相应依赖的边界处（应用 crate、Web 层、中间件）。参见设计文档 §7、§11。

## 1. 字段命名

所有字段名统一以 `error.` 命名空间为前缀，并以常量形式固化在
[`infinity_error::field`] 模块中，禁止各 crate 各自硬编码字符串：

| 常量 | 字段名 | 取值来源 | 类型 | 基数 |
| ---- | ------ | -------- | ---- | ---- |
| `field::KIND` | `error.kind` | `ErrorKind::code()` | string | 有界（`ErrorKind::ALL`，当前 16） |
| `field::STATUS` | `error.status` | `InfinityError::status_code()` | u16 | 有界（少数几个） |
| `field::CLASS` | `error.class` | `ErrorClass::as_str()` | `client` / `server` | 2 |
| `field::MESSAGE` | `error.message` | 错误的 `Display` | string | 无界 |
| `field::ROOT_CAUSE` | `error.root_cause` | `InfinityError::root_cause()` | string | 无界 |
| `field::CHAIN` | `error.chain` | `chain()` / `chain_string()` | string[] / string | 无界 |

**基数**一栏决定字段能否用作 metrics label：只有「有界」字段可以做 label，
「无界」字段仅用于日志/trace 的自由文本。

## 2. 结构化日志规范

在处理边界（请求中间件、后台任务的顶层、`main`）记录一次错误时，**至少**携带
`error.kind`、`error.status`、`error.class`、`error.message` 四个字段；建议同时携带
`error.root_cause`（便于跳过中间层直接定位根因）与请求关联字段 `request_id`
（见 infinity-web README）。

- **日志级别**：`error.class == "server"`（5xx）用 `error`；`error.class == "client"`
  （4xx）用 `warn` 或 `info`，避免调用方输入错误刷爆错误日志与告警。
- **不要**把 `error.message` / `error.root_cause` / `error.chain` 当作 metrics label
  或高基数索引字段——它们含动态内容（ID、路径、SQL 片段），会导致标签爆炸。

`tracing` 示例（在持有 `tracing` 依赖的 crate 中）：

```rust,ignore
use infinity_error::{field, ErrorClass, InfinityError};

fn log_error(err: &InfinityError, request_id: &str) {
    let record = |level| {
        // 字段名全部取自 field 常量，保证跨 crate 一致
        match level {
            ErrorClass::Server => tracing::error!(
                { field::KIND } = err.code(),
                { field::STATUS } = err.status_code(),
                { field::CLASS } = err.class().as_str(),
                { field::ROOT_CAUSE } = %err.root_cause(),
                request_id,
                { field::MESSAGE } = %err,
            ),
            ErrorClass::Client => tracing::warn!(
                { field::KIND } = err.code(),
                { field::STATUS } = err.status_code(),
                { field::CLASS } = err.class().as_str(),
                request_id,
                { field::MESSAGE } = %err,
            ),
        }
    };
    record(err.class());
}
```

## 3. trace / span 规范

当一次操作在 span 内失败时，建议在返回错误前把以下字段记录到当前 span，使 trace
后端（Jaeger / Tempo 等）可以按错误分类过滤 span：

- `error = true`（OpenTelemetry 约定的布尔标记）
- `error.kind`、`error.status`、`error.class`——用于筛选与分面
- `error.message`、`error.root_cause`——用于人工排查
- 完整因果链写入 `error.chain`：字段较多的后端用 `chain()` 收集成数组，只支持标量的
  后端用 `chain_string()` 拼成单行

```rust,ignore
use infinity_error::{field, InfinityError};

fn annotate_span(err: &InfinityError) {
    let span = tracing::Span::current();
    span.record("error", true);
    span.record(field::KIND, err.code());
    span.record(field::CLASS, err.class().as_str());
    span.record(field::CHAIN, err.chain_string().as_str());
}
```

## 4. metrics label 规范

错误计数器只使用**有界基数**字段作为 label：

- 主分类 label 用 `error.kind`，取值来自 `ErrorKind::code()`。标签集合有界且稳定，
  可用 `ErrorKind::ALL` 在启动时预先注册全部取值。
- 粗粒度归责 label 用 `error.class`（`client` / `server`），只有两个取值，适合快速
  计算 5xx 比例、驱动 SLO 告警。
- 需要时可加 `error.status`（HTTP 状态码，取值有限）。
- **严禁**使用 `error.message` / `error.root_cause` / `error.chain` 作为 label。

```rust,ignore
use infinity_error::{field, InfinityError};

fn count_error(err: &InfinityError) {
    metrics::counter!(
        "infinity_errors_total",
        field::KIND => err.code(),
        field::CLASS => err.class().as_str(),
    )
    .increment(1);
}

// 启动时用 ErrorKind::ALL 预注册全部 kind label，避免首次出现某类错误时才创建时间序列。
fn preregister() {
    for kind in infinity_error::ErrorKind::ALL {
        metrics::counter!("infinity_errors_total", field::KIND => kind.code());
    }
}
```

## 5. 相关 API

| 用途 | API |
| ---- | --- |
| 分类码 / metrics 主 label | [`ErrorKind::code`] · [`InfinityError::code`] |
| 全部分类（预注册 label） | [`ErrorKind::ALL`] |
| HTTP 状态码 | [`InfinityError::status_code`] |
| 粗粒度归责分类 | [`InfinityError::class`] · [`ErrorClass::as_str`] |
| 根因 | [`InfinityError::root_cause`] |
| 完整错误链 | [`InfinityError::chain`] · [`InfinityError::chain_string`] |
| 字段名常量 | [`infinity_error::field`] |

[`infinity_error::field`]: ../src/lib.rs
[`ErrorKind::code`]: ../src/lib.rs
[`ErrorKind::ALL`]: ../src/lib.rs
[`InfinityError::code`]: ../src/lib.rs
[`InfinityError::status_code`]: ../src/lib.rs
[`InfinityError::class`]: ../src/lib.rs
[`ErrorClass::as_str`]: ../src/lib.rs
[`InfinityError::root_cause`]: ../src/lib.rs
[`InfinityError::chain`]: ../src/lib.rs
[`InfinityError::chain_string`]: ../src/lib.rs

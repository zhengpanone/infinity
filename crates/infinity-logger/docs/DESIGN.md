# Infinity Logger Design

> **Project**: infinity-logger
> **Version**: v2.0
> **Status**: Frozen API
> **Edition**: Rust 2024
> **MSRV**: Rust 1.88+
> **License**: Apache-2.0 OR MIT

---

# 1. Vision

## 1.1 Background

随着 Infinity 平台不断扩展（AI、ERP、WMS、网关、认证中心、配置中心等），系统需要统一的日志基础设施，以满足：

* 统一日志格式
* 统一初始化方式
* 统一配置管理
* 链路追踪
* 多协议支持（HTTP/gRPC）
* 可观测性（Observability）

Rust 官方已经提供了完整的日志生态：

* tracing
* tracing-core
* tracing-subscriber
* tracing-appender
* tracing-opentelemetry

因此，Infinity Logger **不会重新实现 Logging Framework**，而是在官方生态之上提供企业级工程能力。

---

## 1.2 Goals

Infinity Logger 的目标：

* Builder API
* Configuration Driven
* Feature Gate
* Zero Business Intrusion
* OpenTelemetry Ready
* Workspace Friendly
* Production Ready
* crates.io Ready

---

## 1.3 Non Goals

Infinity Logger 不负责：

* ELK
* Loki Client
* Kafka Appender
* ClickHouse
* Elasticsearch
* 日志分析
* 日志查询
* 日志采集 Agent

这些属于日志平台。

---

# 2. Design Principles

## Principle 1

**Official First**

优先采用 tracing 官方 API。

不重新实现：

* Subscriber
* Layer
* Event Formatter
* Registry

Infinity Logger 负责：

* Configuration
* Initialization
* Integration

---

## Principle 2

**Configuration Driven**

所有行为来自 LoggerConfig。

Builder 只是 Config Builder。

```
Builder
      │
      ▼
 LoggerConfig
      │
      ▼
 Initialization
```

---

## Principle 3

**Composition over Inheritance**

所有功能采用组合。

例如：

```
Console

+

Rolling File

+

JSON

+

OTLP
```

而不是继承。

---

## Principle 4

**Single Responsibility**

每个模块只有一个职责。

---

## Principle 5

**Feature Gate**

高级功能必须通过 Cargo Feature 启用。

默认只启用 Console。

---

# 3. High Level Architecture

```
                 LoggerBuilder
                       │
                       ▼
                 LoggerConfig
                       │
                       ▼
              Initialization Engine
                       │
         ┌─────────────┼──────────────┐
         ▼             ▼              ▼
      Filter        Formatter       Writer
         │             │              │
         └─────────────┼──────────────┘
                       ▼
             tracing_subscriber
                       │
                       ▼
                  Subscriber
```

---

# 4. Project Layout

```
crates/infinity-logger
│
├── Cargo.toml
├── README.md
├── DESIGN.md
│
├── examples
│   ├── basic.rs
│   ├── file.rs
│   ├── json.rs
│   ├── reload.rs
│   ├── axum.rs
│   └── tonic.rs
│
├── src
│
├── lib.rs
├── builder.rs
├── config.rs
├── error.rs
├── init.rs
│
├── layer
│   ├── mod.rs
│   ├── text.rs
│   ├── json.rs
│   └── filter.rs
│
├── rolling
│   ├── mod.rs
│   └── factory.rs
│
├── middleware
│   ├── mod.rs
│   ├── request_id.rs
│   ├── axum.rs
│   └── tonic.rs
│
├── telemetry
│   ├── mod.rs
│   └── otlp.rs
│
└── tests
```

---

# 5. Module Responsibilities

| Module     | Responsibility            |
| ---------- | ------------------------- |
| builder    | Builder API               |
| config     | LoggerConfig              |
| error      | Error Definition          |
| init       | Subscriber Initialization |
| layer      | Layer Construction        |
| rolling    | Rolling File Factory      |
| middleware | HTTP/gRPC Integration     |
| telemetry  | OpenTelemetry             |
| tests      | Unit & Integration Tests  |

---

# 6. Public API

唯一入口：

```rust
LoggerBuilder::default()
```

初始化：

```rust
LoggerBuilder::default()
    .level(LogLevel::Info)
    .console(true)
    .file(true)
    .directory("./logs")
    .filename("application")
    .rotation(Rotation::Daily)
    .json(false)
    .init()?;
```

配置初始化：

```rust
LoggerBuilder::from_config(config)
    .init()?;
```

配置文件：

```rust
LoggerBuilder::from_file("logger.toml")
    .init()?;
```

未来支持：

```rust
LoggerBuilder::reload(Level::Debug)?;
```

---

# 7. LoggerConfig

```
LoggerConfig
│
├── level
├── console
├── file
├── json
├── format
├── filter
└── telemetry
```

所有 Builder 方法仅修改 LoggerConfig。

---

# 8. Initialization Flow

```
LoggerBuilder
        │
        ▼
 LoggerConfig
        │
        ▼
 Validate Config
        │
        ▼
 Build EnvFilter
        │
        ▼
 Build Layer
        │
        ▼
 Registry
        │
        ▼
 try_init()
```

---

# 9. Configuration File

```toml
level = "info"

[console]
enabled = true
ansi = true

[file]
enabled = false
directory = "./logs"
filename = "application"
rotation = "daily"

[json]
enabled = false
```

未来支持：

```toml
[telemetry]
enabled = true
endpoint = "http://localhost:4317"
```

---

# 10. Cargo Features

```toml
default = ["console"]

console = []

file = [
    "dep:tracing-appender"
]

json = []

reload = []

otel = [
    "dep:opentelemetry",
    "dep:tracing-opentelemetry"
]

axum = [
    "dep:tower-http"
]

tonic = [
    "dep:tonic"
]
```

---

# 11. Error Handling

统一错误类型：

```rust
LoggerError
```

分类：

* Configuration Error
* IO Error
* Parse Error
* Subscriber Error
* Reload Error
* Telemetry Error

所有公开 API 返回：

```rust
Result<T>
```

禁止：

* panic!
* unwrap()
* expect()

（测试代码除外。）

---

# 12. Testing Strategy

测试目录：

```
tests
├── builder.rs
├── config.rs
├── console.rs
├── file.rs
├── json.rs
├── reload.rs
├── middleware.rs
└── integration.rs
```

覆盖：

* Builder
* Config
* Layer
* File Rotation
* JSON
* Middleware
* Reload
* Integration

目标覆盖率：

> ≥ 90%

---

# 13. Documentation Strategy

每个公开 API：

* RustDoc
* Example
* Panic Safety
* Error Description

README 包括：

* Quick Start
* Configuration
* Examples
* Features

---

# 14. Coding Standards

必须通过：

```bash
cargo fmt

cargo check

cargo clippy --all-features -- -D warnings

cargo test

cargo doc --no-deps
```

禁止：

* unwrap()
* expect()
* todo!()
* unreachable!()

（测试代码除外。）

---

# 15. Versioning Policy

遵循 Semantic Versioning。

```
MAJOR.MINOR.PATCH
```

例如：

```
1.0.0
```

冻结 Public API。

Breaking Change：

必须升级 Major。

---

# 16. Compatibility

| Component          | Version  |
| ------------------ | -------- |
| Rust               | >=1.88   |
| Edition            | 2024     |
| tracing            | Stable   |
| tracing-subscriber | Stable   |
| tracing-appender   | Optional |

---

# 17. Roadmap

## Milestone 1

* Console
* Builder
* Config

---

## Milestone 2

* Rolling File

---

## Milestone 3

* JSON

---

## Milestone 4

* Dynamic Reload

---

## Milestone 5

* RequestId

---

## Milestone 6

* Axum Middleware

---

## Milestone 7

* Tonic Middleware

---

## Milestone 8

* OpenTelemetry

---

# 18. Future Evolution

未来扩展：

* Profile（dev/prod/test）
* Environment Variables
* TOML Merge
* YAML Support
* Dynamic Config Center
* Metrics Integration
* Distributed Trace Context
* Multi-Tenant Metadata

以上扩展必须保持 Public API 向后兼容。

---

# 19. Architecture Decision Record (ADR)

所有影响 Public API 或架构的重要变更，必须：

1. 更新 DESIGN.md。
2. 记录 ADR。
3. 审查兼容性。
4. 确认不会破坏现有用户代码。

---

# 20. Conclusion

Infinity Logger 是 Infinity Workspace 的统一日志基础设施。

设计原则：

* Official First
* Configuration Driven
* Composition over Inheritance
* Feature Gate
* Production Ready

本设计文档冻结后，后续开发以本文件为唯一架构依据，不再随意修改 Public API。

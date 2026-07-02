下面这份 **ARCHITECTURE.md** 我建议作为 **infinity-logger** 的架构文档。

它与 **DESIGN.md** 不同。

* **DESIGN.md**：说明**为什么这样设计（Why）**
* **ARCHITECTURE.md**：说明**如何实现（How）**

两份文档共同组成整个项目的设计规范。

---

# Infinity Logger Architecture

> **Project:** infinity-logger
> **Version:** v1.0
> **Status:** Frozen
> **Related:** DESIGN.md

---

# 1. Purpose

本文档描述 **Infinity Logger** 的内部架构、模块职责、调用流程以及各组件之间的关系。

本文件主要面向：

* 项目维护者
* 贡献者（Contributors）
* 架构设计人员

如果需要了解项目目标，请参考 **DESIGN.md**。

---

# 2. Overall Architecture

```text
                     Application
                           │
                           ▼
                    LoggerBuilder
                           │
                           ▼
                    LoggerConfig
                           │
                           ▼
                      Validation
                           │
                           ▼
                    Initialization
                           │
                           ▼
                  tracing_subscriber
                           │
      ┌────────────────────┼────────────────────┐
      ▼                    ▼                    ▼
 EnvFilter            Format Layer         Writer
      │                    │                    │
      └────────────────────┼────────────────────┘
                           ▼
                      Subscriber
                           │
                           ▼
                      Global Default
```

整个初始化过程只有一条路径。

Builder 永远不会直接创建 Subscriber。

---

# 3. Package Structure

```text
src
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
└── util
    ├── mod.rs
    ├── env.rs
    ├── time.rs
    └── path.rs
```

---

# 4. Dependency Graph

```text
Application
      │
      ▼
 LoggerBuilder
      │
      ▼
 LoggerConfig
      │
      ▼
    init.rs
      │
      ▼
layer/*

rolling/*

telemetry/*
      │
      ▼
tracing_subscriber
```

依赖方向必须保持单向。

禁止：

```text
layer

↓

builder
```

这种反向依赖。

---

# 5. Module Responsibilities

## builder

职责：

* Builder API
* 修改 LoggerConfig
* 参数校验
* 调用初始化

不负责：

* Layer
* Registry
* Subscriber

---

## config

职责：

保存全部配置。

所有 Builder 方法最终都会修改 Config。

---

## init

整个项目唯一允许出现：

```rust
tracing_subscriber::registry()
```

的位置。

职责：

* 校验配置
* 创建 Filter
* 创建 Layer
* 创建 Writer
* 初始化 Subscriber

---

## layer

职责：

构建 tracing Layer。

例如：

```text
Text Layer

JSON Layer

Filter Layer
```

不负责：

* 初始化

---

## rolling

职责：

创建：

* RollingFileAppender
* NonBlocking
* WorkerGuard

不会创建 Layer。

---

## middleware

职责：

框架集成：

* Axum
* Tonic

负责：

* RequestId
* Span
* Trace Context

---

## telemetry

负责：

OpenTelemetry。

包括：

* OTLP
* Trace Export

---

## util

公共工具。

例如：

* Environment
* Time
* File Path

---

# 6. Initialization Sequence

```text
Application

↓

LoggerBuilder::default()

↓

Builder Methods

↓

LoggerConfig

↓

Builder::init()

↓

init(config)

↓

Validate Config

↓

Build EnvFilter

↓

Build Layer

↓

Build Writer

↓

Registry

↓

try_init()
```

Builder 永远不会直接访问 Registry。

---

# 7. Runtime Architecture

初始化完成后：

```text
Tracing Macro

↓

Subscriber

↓

Filter

↓

Formatter

↓

Writer

↓

Console/File/JSON
```

日志事件全部经过同一条链路。

---

# 8. Configuration Flow

```text
Default Config

↓

Builder

↓

logger.toml

↓

Environment Variables

↓

Final LoggerConfig
```

配置优先级：

```text
Builder

>

Environment

>

Configuration File

>

Default
```

后续可以支持配置中心，但保持相同优先级原则。

---

# 9. Error Flow

所有错误统一返回：

```rust
Result<T>
```

错误来源：

```text
Configuration

↓

IO

↓

Parse

↓

Subscriber

↓

Telemetry
```

统一转换：

```rust
LoggerError
```

禁止暴露第三方错误类型。

---

# 10. Layer Composition

Console：

```text
Registry

↓

EnvFilter

↓

Text Layer

↓

Stdout
```

---

File：

```text
Registry

↓

EnvFilter

↓

Text Layer

↓

Rolling Writer
```

---

JSON：

```text
Registry

↓

EnvFilter

↓

JSON Layer

↓

Rolling Writer
```

---

OTLP：

```text
Registry

↓

EnvFilter

↓

OTLP Layer

↓

Exporter
```

所有输出共享同一个 Registry。

---

# 11. File Logging Pipeline

```text
Tracing Event

↓

Formatter

↓

NonBlocking

↓

Worker Thread

↓

RollingFileAppender

↓

Disk
```

采用异步写盘。

避免阻塞业务线程。

---

# 12. Middleware Flow

HTTP：

```text
HTTP Request

↓

RequestId

↓

Span

↓

Business Logic

↓

Response

↓

Access Log
```

---

gRPC：

```text
Request

↓

Interceptor

↓

Span

↓

Business Logic

↓

Response
```

---

# 13. Telemetry Flow

```text
Tracing Event

↓

Span

↓

OpenTelemetry Layer

↓

OTLP Exporter

↓

Collector

↓

Jaeger / Tempo
```

---

# 14. Concurrency Model

Logger 初始化：

```text
Single Initialization
```

运行期间：

```text
Read Only
```

Reload：

```text
Atomic Update
```

所有组件必须：

* Send
* Sync

---

# 15. Feature Dependency

```text
console

↓

base

----------------

file

↓

rolling

----------------

json

↓

layer

----------------

reload

↓

tracing_subscriber::reload

----------------

otel

↓

opentelemetry

----------------

axum

↓

tower-http

----------------

tonic

↓

tonic
```

Feature 不允许循环依赖。

---

# 16. Testing Architecture

```text
Unit Test

↓

Integration Test

↓

Example Test

↓

Doc Test
```

目标：

* Builder
* Config
* Layer
* Rolling
* Middleware
* Telemetry

全部覆盖。

---

# 17. Performance Considerations

设计原则：

* 零拷贝优先
* 非阻塞写文件
* 避免动态分配
* 避免锁竞争
* 尽量使用静态分发
* 最小化初始化开销

---

# 18. Extension Points

允许扩展：

* 新 Layer
* 新 Middleware
* 新 Telemetry Exporter
* 新 Writer
* 新 Config Loader

要求：

不修改 Public API。

---

# 19. Backward Compatibility

以下内容一旦发布不得轻易修改：

* LoggerBuilder API
* LoggerConfig
* Error Type
* Feature 名称

新增能力采用：

* 新字段
* 新 Feature
* 新模块

保证向后兼容。

---

# 20. Architecture Rules

所有贡献者必须遵守：

1. Builder 不依赖 Layer。
2. Layer 不依赖 Builder。
3. Config 不依赖 tracing。
4. init.rs 是唯一允许初始化 Subscriber 的模块。
5. 所有错误统一转换为 LoggerError。
6. 所有公共 API 必须编写 RustDoc。
7. 新增 Feature 不允许破坏默认行为。
8. Public API 修改必须同步更新 DESIGN.md 与 ARCHITECTURE.md。

---

# 21. Summary

Infinity Logger 采用 **Builder + Configuration + Initialization** 三层架构：

* **Builder**：构建配置。
* **Config**：保存配置。
* **Initialization**：组装并初始化 `tracing_subscriber`。

各模块职责单一、依赖单向、可扩展且保持向后兼容，为 Infinity Workspace 提供统一、稳定、可维护的日志基础设施。

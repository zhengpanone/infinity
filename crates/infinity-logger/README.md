# 设计目标

DESIGN.md：说明为什么这样设计（Why）
ARCHITECTURE.md：说明如何实现（How）

整个 `logger` 需要满足下面几个原则：

* **Builder 只负责配置**
* **Logger 只负责生命周期管理**
* **Layer 只负责日志格式**
* **Writer 只负责输出**
* **Middleware 只负责上下文**
* **Config 可以来自 Builder、TOML、环境变量**
* **后续支持热更新**
* **后续支持 OTLP**
* **所有模块单一职责**

---

# 最终目录

```text
logger/
├── Cargo.toml
├── README.md
│
├── src
│   ├── lib.rs                # 对外导出
│   ├── logger.rs             # Logger 生命周期
│   ├── builder.rs            # Builder
│   ├── config.rs             # 配置
│   ├── error.rs              # 错误
│   │
│   ├── init.rs               # 初始化入口 ⭐⭐⭐
│   │
│   ├── layer/
│   │   ├── mod.rs
│   │   ├── console.rs
│   │   ├── file.rs
│   │   ├── json.rs
│   │   └── otel.rs
│   │
│   ├── formatter/
│   │   ├── mod.rs
│   │   ├── console.rs
│   │   ├── json.rs
│   │   └── timer.rs
│   │
│   ├── writer/
│   │   ├── mod.rs
│   │   ├── rolling.rs
│   │   ├── stdout.rs
│   │   └── stderr.rs
│   │
│   ├── middleware/
│   │   ├── mod.rs
│   │   ├── axum.rs
│   │   ├── grpc.rs
│   │   └── request_id.rs
│   │
│   └── util/
│       ├── mod.rs
│       ├── time.rs
│       ├── thread.rs
│       └── hostname.rs
```

这一版目录基本不会再变化。

---

# 模块关系

```text
                   Logger
                      │
                      ▼
                LoggerBuilder
                      │
                      ▼
                LoggerConfig
                      │
                      ▼
             SubscriberFactory
                      │
        ┌─────────────┼──────────────┐
        ▼             ▼              ▼
   Formatter      Writer         Middleware
        │             │              │
        └─────────────┼──────────────┘
                      ▼
             tracing_subscriber
                      │
                      ▼
                  Subscriber
```

---

# 初始化流程

以后真正初始化只有这一条路径：

```rust
Logger::builder()
    .level(LogLevel::Debug)
    .file(true)
    .json(false)
    .init()?;
```

内部流程：

```text
Builder
    │
    ▼
LoggerConfig
    │
    ▼
SubscriberFactory
    │
    ▼
Console Subscriber
    │
    ▼
File Writer
    │
    ▼
Subscriber::try_init()
```

Builder 不会直接接触 `tracing_subscriber`。

---

# 为什么增加 SubscriberFactory？

这是整个项目最重要的类。

职责：

* 创建 EnvFilter
* 创建 Writer
* 创建 Formatter
* 创建 Subscriber
* 注册 Subscriber
* 后续支持 ReloadHandle

Builder 完全不知道 tracing 的细节。

---

# Writer 独立出来

很多教程都会这样写：

```rust
fmt::layer()
    .with_writer(file)
```

但是以后：

* stdout
* stderr
* rolling file
* 多文件
* TCP
* Kafka
* Loki

都会涉及 Writer。

所以单独抽象：

```text
Writer
│
├── StdoutWriter
├── RollingWriter
├── JsonWriter
└── MultiWriter
```

以后 Builder：

```rust
.file(true)
```

只是修改 Config。

---

# Formatter 独立

以后 Console：

```text
INFO hello
```

JSON：

```json
{
  "level":"INFO"
}
```

不是 Writer 决定。

而是：

```text
Formatter
```

决定。

所以：

```text
Formatter
│
├── ConsoleFormatter
├── JsonFormatter
└── TimerFormatter
```

---

# Middleware

这里只负责：

```text
RequestId

TraceId

Span

UserId

TenantId
```

以后：

Axum：

```rust
.layer(RequestIdLayer)
```

Tonic：

```rust
.interceptor(RequestInterceptor)
```

不会影响 Logger。

---

# Telemetry

这里以后完全独立：

```text
Telemetry
│
├── OTLP
├── Jaeger
├── Tempo
├── Zipkin
```

Builder：

```rust
.telemetry(true)
```

即可。

---

# Builder 最终效果

```rust
Logger::builder()
    .level(LogLevel::Debug)
    .console(true)
    .file(true)
    .json(false)
    .directory("./logs")
    .filename("infinity")
    .rotation(Rotation::Daily)
    .ansi(true)
    .with_target(true)
    .with_thread_name(true)
    .with_thread_id(true)
    .with_file(true)
    .with_line_number(true)
    .init()?;
```

Builder 永远不接触 tracing。

---

# 我建议再升级一个层次（也是我最推荐的方案）

如果目标是把 **Infinity** 打造成企业级平台，我建议 `logger` 不要直接依赖 `tracing_subscriber`。

可以新增一个 **backend** 抽象层：

```text
Logger
     │
     ▼
Backend Trait
     │
 ┌───┴────────────┐
 ▼                ▼
Tracing      OpenTelemetry
```

定义：

```rust
pub trait LogBackend {
    fn init(&self, config: &LoggerConfig) -> Result<()>;
}
```

默认实现：

```text
TracingBackend
```

以后如果某些场景需要直接接入 OpenTelemetry SDK，而不经过 `tracing_subscriber`，甚至需要适配其他日志实现，就无需修改 `LoggerBuilder` 和业务代码，只需新增一个 Backend 实现。

---

**我建议从这里开始重新实现代码，而不是继续修补之前的版本。**
接下来我们将按照这份架构，从 **`subscriber/factory.rs`** 开始，一步一步实现，保证整个项目最终能够直接 `cargo test`、`cargo clippy`、`cargo doc` 全部通过，并达到可开源发布的质量。

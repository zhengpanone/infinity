# Infinity Logger

[![Crates.io](https://img.shields.io/crates/v/infinity-logger.svg)](https://crates.io/crates/infinity-logger)
[![Documentation](https://docs.rs/infinity-logger/badge.svg)](https://docs.rs/infinity-logger)
[![License](https://img.shields.io/badge/license-Apache--2.0%20OR%20MIT-blue.svg)](LICENSE)

企业级日志组件，为 [Infinity Workspace](https://github.com/zhengpanone/infinity) 提供统一的日志基础设施。

基于 Rust 官方 [tracing](https://github.com/tokio-rs/tracing) 生态构建，提供开箱即用的企业级日志能力。

## ✨ 特性

- 🚀 **开箱即用** - Builder API，零配置快速启动
- 📝 **多种格式** - Console、JSON、结构化日志
- 📁 **文件轮转** - 支持按日、按小时的日志轮转
- 🔍 **链路追踪** - OpenTelemetry 集成
- 🌐 **框架集成** - Axum、Tonic 中间件支持
- ⚙️ **灵活配置** - Builder、TOML、环境变量
- 🎯 **特性门控** - 按需启用功能，减小依赖

## 📦 安装

```toml
[dependencies]
infinity-logger = "0.1"
```

启用文件日志：

```toml
[dependencies]
infinity-logger = { version = "0.1", features = ["file"] }
```

启用所有功能：

```toml
[dependencies]
infinity-logger = { version = "0.1", features = ["full"] }
```

## 🚀 快速开始

### 基础用法

```rust
use infinity_logger::Logger;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    Logger::builder()
        .init()?;

    tracing::info!("Hello, Infinity!");
    Ok(())
}
```

### 自定义配置

```rust
use infinity_logger::{Logger, config::LogLevel};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    Logger::builder()
        .level(LogLevel::Debug)
        .console(true)
        .file(true)
        .directory("./logs")
        .filename("app")
        .init()?;

    tracing::debug!("Debug message");
    tracing::info!("Application started");
    Ok(())
}
```

### 结构化日志

启用 `json` feature：

```rust
use infinity_logger::Logger;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    Logger::builder()
        .json(true)
        .init()?;

    tracing::info!(user_id = 123, "User logged in");
    Ok(())
}
```

输出：

```json
{
  "timestamp": "2026-07-03T12:00:00Z",
  "level": "INFO",
  "target": "app",
  "fields": {
    "user_id": 123,
    "message": "User logged in"
  }
}
```

## 📋 功能特性

### 可用 Features

| Feature   | 描述                   | 默认 |
|-----------|------------------------|------|
| `console` | 控制台日志输出         | ✅   |
| `file`    | 文件日志与轮转         | ❌   |
| `json`    | JSON 格式日志          | ❌   |
| `otel`    | OpenTelemetry 集成     | ❌   |
| `axum`    | Axum 中间件            | ❌   |
| `grpc`    | gRPC/Tonic 中间件      | ❌   |
| `full`    | 启用所有功能           | ❌   |

### 日志轮转

```rust
use infinity_logger::{Logger, config::Rotation};

Logger::builder()
    .file(true)
    .directory("./logs")
    .filename("application")
    .rotation(Rotation::Daily)  // 每日轮转
    .init()?;
```

支持的轮转策略：
- `Rotation::Never` - 不轮转
- `Rotation::Minutely` - 每分钟（测试用）
- `Rotation::Hourly` - 每小时
- `Rotation::Daily` - 每日

### Axum 集成

启用 `axum` feature：

```rust
use axum::{Router, routing::get};
use infinity_logger::Logger;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    Logger::builder().init()?;

    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;
    axum::serve(listener, app).await?;
    Ok(())
}
```

### OpenTelemetry

启用 `otel` feature：

```rust
use infinity_logger::Logger;

Logger::builder()
    .init()?;

tracing::info!("This trace will be exported to OpenTelemetry collectors");
```

## 📚 文档

- [设计文档](docs/DESIGN.md) - 设计理念与目标
- [架构文档](docs/ARCHITECTURE.md) - 内部架构说明
- [配置指南](docs/CONFIGURATION.md) - 详细配置说明
- [测试指南](docs/TESTING.md) - 测试策略
- [贡献指南](docs/CONTRIBUTING.md) - 如何参与贡献
- [路线图](docs/ROADMAP.md) - 开发计划

## 🎯 设计原则

1. **Official First** - 优先使用 `tracing` 官方能力，不重复造轮子
2. **Configuration Driven** - 配置驱动，行为可预测
3. **Composition over Inheritance** - 组合优于继承
4. **Single Responsibility** - 每个模块职责单一
5. **Feature Gate** - 高级功能按需启用

## 🤝 贡献

欢迎贡献！请阅读 [CONTRIBUTING.md](docs/CONTRIBUTING.md) 了解详情。

提交前请确保：

```bash
cargo fmt
cargo clippy --all-features -- -D warnings
cargo test --all-features
cargo doc --no-deps
```

## 📄 许可证

本项目采用以下任一许可证：

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT license ([LICENSE-MIT](LICENSE-MIT))

由你选择。

## 🔗 相关项目

- [tracing](https://github.com/tokio-rs/tracing) - Rust 官方追踪框架
- [tracing-subscriber](https://github.com/tokio-rs/tracing) - Subscriber 实现
- [OpenTelemetry](https://opentelemetry.io/) - 可观测性标准

---

**Made with ❤️ for the Infinity Workspace**

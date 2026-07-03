# Frequently Asked Questions (FAQ)

> **Project:** infinity-logger  
> **Version:** v1.0

---

## 目录

- [通用问题](#通用问题)
- [配置问题](#配置问题)
- [功能问题](#功能问题)
- [性能问题](#性能问题)
- [集成问题](#集成问题)
- [故障排查](#故障排查)

---

## 通用问题

### Q: Infinity Logger 与 tracing 的关系？

**A:** Infinity Logger 是基于 `tracing` 构建的企业级日志组件，不是替代品。

- **tracing** - 提供核心日志和追踪能力
- **infinity-logger** - 提供开箱即用的配置、初始化和集成

你仍然使用 `tracing` 的宏（`info!`, `debug!` 等）记录日志。

---

### Q: 为什么选择 Infinity Logger 而不是直接用 tracing？

**A:** 如果你需要：

- ✅ 开箱即用的配置
- ✅ 统一的初始化方式
- ✅ 文件轮转支持
- ✅ JSON 格式化
- ✅ 中间件集成（Axum、Tonic）
- ✅ OpenTelemetry 集成

那么 Infinity Logger 能节省你的时间。

如果只需要简单的控制台日志，直接用 `tracing-subscriber::fmt::init()` 即可。

---

### Q: Infinity Logger 稳定吗？

**A:** 当前版本：

- **0.x** - 快速迭代阶段，API 可能变动
- **1.0+** - API 稳定，保证向后兼容

建议生产环境使用 1.0 及以上版本。

---

### Q: 支持哪些 Rust 版本？

**A:** 

- **MSRV**: Rust 1.88+
- **Edition**: 2024
- **推荐**: 使用最新 stable 版本

---

### Q: 许可证是什么？

**A:** 双许可：

- Apache License 2.0
- MIT License

你可以选择其中任一许可证。

---

## 配置问题

### Q: 如何同时输出到控制台和文件？

**A:**

```rust
use infinity_logger::Logger;

Logger::builder()
    .console(true)  // 控制台
    .file(true)     // 文件
    .directory("./logs")
    .init()?;
```

两者可以同时启用。

---

### Q: 如何为不同环境使用不同配置？

**A:**

```rust
fn init_logger() -> Result<(), infinity_logger::LoggerError> {
    let config = if cfg!(debug_assertions) {
        // 开发环境
        Logger::builder()
            .level(LogLevel::Debug)
            .console(true)
            .file(false)
    } else {
        // 生产环境
        Logger::builder()
            .level(LogLevel::Info)
            .console(false)
            .file(true)
            .json(true)
    };
    
    config.init()
}
```

---

### Q: 如何通过环境变量覆盖配置？

**A:** 使用 `RUST_LOG` 环境变量：

```bash
# 设置全局级别
RUST_LOG=debug cargo run

# 设置模块级别
RUST_LOG=info,my_app=debug cargo run
```

`RUST_LOG` 会覆盖 Builder 中的 `level()` 设置。

---

### Q: 如何禁用 ANSI 颜色？

**A:**

方式 1：Builder 配置

```rust
Logger::builder()
    .console(true)
    // ANSI 通过 ConsoleConfig 控制，默认启用
    .init()?;
```

方式 2：环境变量

```bash
NO_COLOR=1 cargo run
```

---

### Q: 文件日志保存在哪里？

**A:** 默认保存在 `./logs` 目录。

自定义：

```rust
Logger::builder()
    .file(true)
    .directory("/var/log/my_app")  // 自定义目录
    .filename("application")        // 文件名前缀
    .init()?;
```

生成文件如：`/var/log/my_app/application.2026-07-03.log`

---

### Q: 如何实现日志轮转？

**A:**

```rust
use infinity_logger::config::Rotation;

Logger::builder()
    .file(true)
    .rotation(Rotation::Daily)  // 每日轮转
    .init()?;
```

支持：
- `Rotation::Never` - 不轮转
- `Rotation::Hourly` - 每小时
- `Rotation::Daily` - 每日

---

## 功能问题

### Q: 如何输出 JSON 格式日志？

**A:**

1. 启用 `json` feature：

```toml
infinity-logger = { version = "0.1", features = ["json"] }
```

2. 配置：

```rust
Logger::builder()
    .json(true)
    .init()?;
```

3. 记录日志：

```rust
tracing::info!(user_id = 123, "User login");
```

输出：

```json
{"timestamp":"...","level":"INFO","fields":{"user_id":123,"message":"User login"}}
```

---

### Q: 如何记录结构化字段？

**A:** 使用 `tracing` 的字段语法：

```rust
tracing::info!(
    user_id = 123,
    method = "POST",
    path = "/api/login",
    duration_ms = 45,
    "Request completed"
);
```

---

### Q: 如何添加全局上下文字段？

**A:** 使用 `tracing::span!`：

```rust
use tracing::info_span;

let _span = info_span!("request", request_id = "abc123").entered();

// 在 span 内的所有日志都会包含 request_id
tracing::info!("Processing request");
tracing::debug!("Step 1 complete");
```

---

### Q: 支持异步日志吗？

**A:** 文件日志已经是异步的（通过 `tracing-appender` 的 `NonBlocking`）。

不需要额外配置。

---

### Q: 如何集成 OpenTelemetry？

**A:**

1. 启用 feature：

```toml
infinity-logger = { version = "0.1", features = ["otel"] }
```

2. 初始化（当前版本自动启用）：

```rust
Logger::builder().init()?;
```

3. 日志会自动导出到 OTLP 收集器。

详细配置见 [OpenTelemetry 文档](https://opentelemetry.io/)。

---

### Q: 如何在 Axum 中使用？

**A:**

1. 启用 feature：

```toml
infinity-logger = { version = "0.1", features = ["axum"] }
```

2. 初始化：

```rust
Logger::builder().init()?;
```

3. 日志会自动关联到 HTTP 请求上下文。

---

## 性能问题

### Q: Infinity Logger 性能如何？

**A:** 基于 `tracing`，性能开销极小：

- **控制台输出**: 每秒百万级日志
- **文件输出**: 异步写入，不阻塞业务线程
- **JSON 格式**: 序列化开销约 10-20%

详见 [BENCHMARK.md](BENCHMARK.md)。

---

### Q: 日志会影响程序性能吗？

**A:** 最佳实践：

1. **使用适当的日志级别**
   ```rust
   // 生产环境使用 Info/Warn
   Logger::builder().level(LogLevel::Info).init()?;
   ```

2. **避免在热路径记录 Debug/Trace**
   ```rust
   // ❌ 不好：循环内大量 debug
   for item in items {
       tracing::debug!("Processing {}", item);
   }
   
   // ✅ 更好：批量记录
   tracing::debug!("Processing {} items", items.len());
   ```

3. **使用字段而非格式化字符串**
   ```rust
   // ✅ 高效：字段在过滤后才计算
   tracing::debug!(count = items.len(), "Processing");
   ```

---

### Q: 文件日志会阻塞吗？

**A:** 不会。文件日志使用 `NonBlocking` writer，写入在后台线程执行。

---

### Q: 如何减少日志文件大小？

**A:**

1. 提高日志级别（减少日志量）
2. 使用日志轮转
3. 定期清理旧日志文件
4. 使用 JSON 格式（便于压缩）

---

## 集成问题

### Q: 可以和 env_logger 一起使用吗？

**A:** 不建议。两者都会初始化全局 subscriber，会冲突。

迁移到 Infinity Logger：

```rust
// 旧代码
env_logger::init();

// 新代码
infinity_logger::Logger::builder().init()?;
```

---

### Q: 如何和第三方库的日志集成？

**A:** `tracing` 通过 `tracing-log` 自动兼容 `log` crate。

第三方库使用 `log` 的日志会自动转发到 `tracing`。

```toml
[dependencies]
tracing-log = "0.2"
```

Infinity Logger 已包含此依赖。

---

### Q: 支持 Tokio Console 吗？

**A:** 当前版本不支持。规划中。

临时方案：手动配置 `console-subscriber`：

```rust
console_subscriber::init();
// 不调用 Logger::builder().init()
```

---

### Q: 如何在库中使用？

**A:** 库不应该初始化 Logger，只记录日志：

```rust
// 库代码
pub fn my_function() {
    tracing::info!("Function called");
}
```

应用程序负责初始化：

```rust
// 应用代码
fn main() {
    infinity_logger::Logger::builder().init().unwrap();
    my_library::my_function();
}
```

---

## 故障排查

### Q: 错误：AlreadyInitialized

**现象**：

```
Error: logger has already been initialized
```

**原因**：`tracing` subscriber 只能初始化一次。

**解决**：

```rust
// 方式 1：忽略错误
Logger::builder().init().ok();

// 方式 2：检查错误
if let Err(e) = Logger::builder().init() {
    if !matches!(e, LoggerError::AlreadyInitialized) {
        panic!("Failed to init logger: {}", e);
    }
}
```

---

### Q: 日志没有输出

**检查清单**：

1. ✅ Logger 是否初始化？
   ```rust
   Logger::builder().init()?;
   ```

2. ✅ 日志级别是否匹配？
   ```rust
   // 如果级别是 Info，Debug 不会输出
   Logger::builder().level(LogLevel::Debug).init()?;
   ```

3. ✅ 环境变量是否过滤？
   ```bash
   # 检查 RUST_LOG
   echo $RUST_LOG
   
   # 重置
   unset RUST_LOG
   ```

4. ✅ Feature 是否启用？
   ```toml
   infinity-logger = { version = "0.1", features = ["file"] }
   ```

---

### Q: 文件没有创建

**检查**：

1. 目录权限
   ```bash
   ls -la ./logs
   ```

2. Feature 是否启用
   ```toml
   features = ["file"]
   ```

3. 配置是否正确
   ```rust
   Logger::builder()
       .file(true)  // ← 必须启用
       .init()?;
   ```

---

### Q: ANSI 颜色不显示

**原因**：

- 终端不支持 ANSI
- 输出重定向到文件
- Windows 旧版本

**解决**：

```bash
# Windows
# 使用 Windows Terminal 或 ConEmu

# Linux/Mac
# 检查 TERM 环境变量
echo $TERM
```

---

### Q: 测试时出现 AlreadyInitialized

**解决**：

方式 1：共享初始化

```rust
use std::sync::Once;

static INIT: Once = Once::new();

fn init_test_logger() {
    INIT.call_once(|| {
        Logger::builder().init().ok();
    });
}

#[test]
fn test_a() {
    init_test_logger();
    // ...
}
```

方式 2：串行测试

```toml
[dev-dependencies]
serial_test = "3"
```

```rust
use serial_test::serial;

#[test]
#[serial]
fn test_a() {
    Logger::builder().init().ok();
}
```

---

### Q: Cargo build 失败

**检查**：

1. Rust 版本
   ```bash
   rustc --version  # 应 ≥ 1.88
   ```

2. Feature 依赖
   ```bash
   cargo tree
   ```

3. 清理缓存
   ```bash
   cargo clean
   cargo build
   ```

---

## 更多问题？

- 📖 查看 [文档](https://docs.rs/infinity-logger)
- 💬 提交 [Issue](https://github.com/zhengpanone/infinity/issues)
- 📧 联系维护者

---

**最后更新**: 2026-07-03

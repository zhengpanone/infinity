# Configuration Guide

> **Project:** infinity-logger  
> **Version:** v1.0  
> **Related:** DESIGN.md, ARCHITECTURE.md

---

## 目录

- [概述](#概述)
- [配置方式](#配置方式)
- [日志级别](#日志级别)
- [控制台日志](#控制台日志)
- [文件日志](#文件日志)
- [JSON 日志](#json日志)
- [格式化配置](#格式化配置)
- [环境变量](#环境变量)
- [完整示例](#完整示例)

---

## 概述

Infinity Logger 支持三种配置方式：

1. **Builder API** - 程序化配置（推荐）
2. **TOML 文件** - 文件配置（待实现）
3. **环境变量** - 运行时覆盖

配置优先级：

```
Builder > Environment Variables > Configuration File > Default
```

---

## 配置方式

### 1. Builder API

最灵活的配置方式：

```rust
use infinity_logger::{Logger, config::{LogLevel, Rotation}};

Logger::builder()
    .level(LogLevel::Debug)
    .console(true)
    .file(true)
    .directory("./logs")
    .filename("application")
    .rotation(Rotation::Daily)
    .json(false)
    .init()?;
```

### 2. TOML 配置文件（规划中）

创建 `logger.toml`：

```toml
level = "info"

[console]
enabled = true
ansi = true
target = true
file = false
line_number = false
thread_name = false
thread_id = false

[file]
enabled = true
directory = "./logs"
filename = "application"
rotation = "daily"
max_file = 30

[json]
enabled = false
pretty = false
flatten = true

[format]
with_target = true
with_file = true
with_line_number = true
with_thread_name = true
with_thread_id = false
with_level = true
with_timer = true
```

加载配置：

```rust
Logger::from_file("logger.toml")?.init()?;
```

### 3. 环境变量

通过 `RUST_LOG` 环境变量控制日志级别：

```bash
RUST_LOG=debug cargo run
RUST_LOG=my_app=trace,hyper=info cargo run
```

---

## 日志级别

### LogLevel 枚举

```rust
pub enum LogLevel {
    Trace,   // 最详细
    Debug,   // 调试信息
    Info,    // 一般信息
    Warn,    // 警告
    Error,   // 错误
}
```

### 设置全局级别

```rust
use infinity_logger::config::LogLevel;

Logger::builder()
    .level(LogLevel::Debug)
    .init()?;
```

### 模块级别控制

通过环境变量：

```bash
# 设置默认为 info，但 my_module 为 debug
RUST_LOG=info,my_module=debug cargo run
```

---

## 控制台日志

### ConsoleConfig

```rust
pub struct ConsoleConfig {
    pub enabled: bool,        // 是否启用
    pub ansi: bool,          // ANSI 颜色
    pub target: bool,        // 显示 target
    pub file: bool,          // 显示文件名
    pub line_number: bool,   // 显示行号
    pub thread_name: bool,   // 显示线程名
    pub thread_id: bool,     // 显示线程 ID
}
```

### 默认配置

```rust
ConsoleConfig {
    enabled: true,
    ansi: true,
    target: true,
    file: false,
    line_number: false,
    thread_name: false,
    thread_id: false,
}
```

### Builder 配置

```rust
Logger::builder()
    .console(true)  // 启用控制台输出
    .init()?;
```

### 输出示例

标准格式：

```
2026-07-03T12:00:00.123456Z  INFO my_app: Application started
2026-07-03T12:00:01.456789Z DEBUG my_app::auth: User login attempt
```

带文件和行号：

```
2026-07-03T12:00:00.123456Z  INFO main.rs:42 my_app: Application started
```

---

## 文件日志

需要启用 `file` feature：

```toml
infinity-logger = { version = "0.1", features = ["file"] }
```

### FileConfig

```rust
pub struct FileConfig {
    pub enabled: bool,        // 是否启用
    pub directory: String,    // 日志目录
    pub filename: String,     // 文件名前缀
    pub rotation: Rotation,   // 轮转策略
    pub max_file: usize,      // 最大保留文件数（待实现）
}
```

### 默认配置

```rust
FileConfig {
    enabled: false,
    directory: "./logs".into(),
    filename: "application".into(),
    rotation: Rotation::Daily,
    max_file: 30,
}
```

### Rotation 策略

```rust
pub enum Rotation {
    Never,      // 不轮转
    Minutely,   // 每分钟（测试用）
    Hourly,     // 每小时
    Daily,      // 每日
}
```

### Builder 配置

```rust
use infinity_logger::config::Rotation;

Logger::builder()
    .file(true)
    .directory("./logs")
    .filename("my_app")
    .rotation(Rotation::Daily)
    .init()?;
```

### 文件命名规则

- **Never**: `my_app.log`
- **Hourly**: `my_app.2026-07-03-12.log`
- **Daily**: `my_app.2026-07-03.log`

### 目录结构示例

```
./logs/
├── my_app.2026-07-01.log
├── my_app.2026-07-02.log
└── my_app.2026-07-03.log
```

---

## JSON 日志

需要启用 `json` feature：

```toml
infinity-logger = { version = "0.1", features = ["json"] }
```

### JsonConfig

```rust
pub struct JsonConfig {
    pub enabled: bool,   // 是否启用
    pub pretty: bool,    // 美化输出
    pub flatten: bool,   // 扁平化字段
}
```

### 默认配置

```rust
JsonConfig {
    enabled: false,
    pretty: false,
    flatten: true,
}
```

### Builder 配置

```rust
Logger::builder()
    .json(true)
    .init()?;
```

### 输出示例

紧凑格式（`pretty: false`）：

```json
{"timestamp":"2026-07-03T12:00:00.123456Z","level":"INFO","target":"my_app","fields":{"message":"User login","user_id":123}}
```

美化格式（`pretty: true`）：

```json
{
  "timestamp": "2026-07-03T12:00:00.123456Z",
  "level": "INFO",
  "target": "my_app",
  "fields": {
    "message": "User login",
    "user_id": 123
  }
}
```

### 结构化日志

```rust
tracing::info!(
    user_id = 123,
    method = "POST",
    path = "/api/login",
    "User login successful"
);
```

JSON 输出：

```json
{
  "timestamp": "2026-07-03T12:00:00.123456Z",
  "level": "INFO",
  "target": "my_app",
  "fields": {
    "user_id": 123,
    "method": "POST",
    "path": "/api/login",
    "message": "User login successful"
  }
}
```

---

## 格式化配置

### FormatConfig

控制日志输出的详细程度：

```rust
pub struct FormatConfig {
    pub with_target: bool,        // 显示 target
    pub with_file: bool,          // 显示文件名
    pub with_line_number: bool,   // 显示行号
    pub with_thread_name: bool,   // 显示线程名
    pub with_thread_id: bool,     // 显示线程 ID
    pub with_level: bool,         // 显示级别
    pub with_timer: bool,         // 显示时间戳
}
```

### 默认配置

```rust
FormatConfig {
    with_target: true,
    with_file: true,
    with_line_number: true,
    with_thread_name: true,
    with_thread_id: false,
    with_level: true,
    with_timer: true,
}
```

### 作用范围

此配置影响：
- 文件日志
- JSON 日志

控制台日志使用 `ConsoleConfig` 中的独立配置。

---

## 环境变量

### RUST_LOG

控制日志级别和过滤器：

```bash
# 全局 info 级别
RUST_LOG=info

# 全局 debug，但 hyper 为 warn
RUST_LOG=debug,hyper=warn

# 仅 my_app 模块 trace
RUST_LOG=my_app=trace

# 复杂过滤
RUST_LOG=warn,my_app::api=debug,my_app::db=trace
```

### 语法规则

```
RUST_LOG = <global_level>
         | <module_filter>[,<module_filter>]*

<module_filter> = <module_path>=<level>
<level> = trace | debug | info | warn | error
```

### 示例

开发环境：

```bash
export RUST_LOG=debug
cargo run
```

生产环境：

```bash
export RUST_LOG=info,my_app::auth=debug
./my_app
```

---

## 完整示例

### 开发环境配置

```rust
use infinity_logger::{Logger, config::LogLevel};

fn init_dev_logger() -> Result<(), infinity_logger::LoggerError> {
    Logger::builder()
        .level(LogLevel::Debug)
        .console(true)
        .init()
}
```

### 生产环境配置

```rust
use infinity_logger::{Logger, config::{LogLevel, Rotation}};

fn init_prod_logger() -> Result<(), infinity_logger::LoggerError> {
    Logger::builder()
        .level(LogLevel::Info)
        .console(false)
        .file(true)
        .directory("/var/log/my_app")
        .filename("application")
        .rotation(Rotation::Daily)
        .json(true)
        .init()
}
```

### 混合输出配置

同时输出到控制台和文件：

```rust
Logger::builder()
    .level(LogLevel::Info)
    .console(true)  // 控制台
    .file(true)     // 文件
    .json(true)     // JSON 格式
    .directory("./logs")
    .filename("app")
    .rotation(Rotation::Daily)
    .init()?;
```

### 最小配置

```rust
Logger::builder().init()?;
```

等价于：

```rust
Logger::builder()
    .level(LogLevel::Info)
    .console(true)
    .file(false)
    .json(false)
    .init()?;
```

---

## 配置验证

Builder 会在 `init()` 时验证配置：

```rust
// ❌ 错误：目录不存在时会尝试创建
Logger::builder()
    .file(true)
    .directory("/root/logs")  // 可能无权限
    .init()?;

// ✅ 正确：使用有效目录
Logger::builder()
    .file(true)
    .directory("./logs")
    .init()?;
```

---

## 配置最佳实践

1. **开发环境**：启用 console，禁用 file
2. **生产环境**：启用 file + json，禁用 console（或仅 error）
3. **日志轮转**：生产环境使用 `Daily`
4. **日志级别**：生产环境使用 `Info` 或 `Warn`
5. **结构化日志**：关键业务日志使用 JSON 格式
6. **敏感信息**：避免记录密码、token 等敏感数据

---

## 配置迁移

### 从 env_logger 迁移

```rust
// 旧代码
env_logger::init();

// 新代码
infinity_logger::Logger::builder().init()?;
```

### 从 tracing_subscriber 迁移

```rust
// 旧代码
tracing_subscriber::fmt::init();

// 新代码
infinity_logger::Logger::builder().init()?;
```

---

## 故障排查

### 日志未输出

检查：
1. 日志级别是否匹配
2. `RUST_LOG` 环境变量是否过滤了日志
3. Logger 是否成功初始化

### 文件未创建

检查：
1. 目录权限
2. 磁盘空间
3. `file` feature 是否启用

### ANSI 颜色不显示

检查：
1. 终端是否支持 ANSI
2. `ansi` 配置是否启用
3. 输出是否重定向到文件

---

## 下一步

- [测试指南](TESTING.md) - 如何测试日志配置
- [架构文档](ARCHITECTURE.md) - 了解内部实现
- [FAQ](FAQ.md) - 常见问题

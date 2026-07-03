# Testing Guide

> **Project:** infinity-logger  
> **Version:** v1.0  
> **Related:** DESIGN.md, ARCHITECTURE.md

---

## 目录

- [测试策略](#测试策略)
- [测试层次](#测试层次)
- [单元测试](#单元测试)
- [集成测试](#集成测试)
- [示例测试](#示例测试)
- [文档测试](#文档测试)
- [性能测试](#性能测试)
- [测试工具](#测试工具)
- [CI/CD](#cicd)

---

## 测试策略

Infinity Logger 采用多层次测试策略：

```
┌─────────────────────────────┐
│       文档测试 (Doc Tests)    │  ← 确保文档示例可运行
├─────────────────────────────┤
│      示例测试 (Examples)      │  ← 确保示例代码正确
├─────────────────────────────┤
│    集成测试 (Integration)     │  ← 测试完整初始化流程
├─────────────────────────────┤
│      单元测试 (Unit Tests)    │  ← 测试各模块独立功能
└─────────────────────────────┘
```

### 测试原则

1. **隔离性** - 每个测试独立运行
2. **可重复** - 相同输入产生相同结果
3. **快速** - 单元测试应在毫秒级完成
4. **覆盖率** - 核心逻辑覆盖率 ≥ 90%
5. **可维护** - 测试代码清晰易懂

---

## 测试层次

### 1. 单元测试

测试单个模块或函数：

```
src/
├── builder.rs       ← 测试 Builder API
├── config.rs        ← 测试配置结构
├── error.rs         ← 测试错误类型
├── layer/           ← 测试各 Layer
└── util/            ← 测试工具函数
```

### 2. 集成测试

测试模块间协作：

```
tests/
├── builder_test.rs       ← Builder 完整流程
├── config_test.rs        ← 配置加载与验证
├── console_test.rs       ← 控制台输出
├── file_test.rs          ← 文件日志
├── json_test.rs          ← JSON 日志
└── integration_test.rs   ← 端到端测试
```

### 3. 示例测试

验证示例代码：

```
examples/
├── basic.rs
├── file.rs
├── json.rs
└── axum.rs
```

### 4. 文档测试

RustDoc 中的代码块：

```rust
/// ```
/// use infinity_logger::Logger;
/// Logger::builder().init()?;
/// # Ok::<(), infinity_logger::LoggerError>(())
/// ```
```

---

## 单元测试

### 测试位置

单元测试位于被测试模块的同一文件中：

```rust
// src/config.rs

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = LoggerConfig::default();
        assert_eq!(config.level, LogLevel::Info);
        assert!(config.console.enabled);
    }

    #[test]
    fn test_log_level_as_str() {
        assert_eq!(LogLevel::Debug.as_str(), "debug");
        assert_eq!(LogLevel::Info.as_str(), "info");
    }
}
```

### 运行单元测试

```bash
# 运行所有单元测试
cargo test --lib

# 运行特定模块
cargo test --lib config::tests

# 显示输出
cargo test --lib -- --nocapture
```

### 测试示例

#### 测试 Builder

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder_default() {
        let builder = LoggerBuilder::new();
        let config = builder.build();
        assert_eq!(config.level, LogLevel::Info);
    }

    #[test]
    fn test_builder_level() {
        let config = LoggerBuilder::new()
            .level(LogLevel::Debug)
            .build();
        assert_eq!(config.level, LogLevel::Debug);
    }

    #[test]
    fn test_builder_file() {
        let config = LoggerBuilder::new()
            .file(true)
            .directory("./test_logs")
            .filename("test")
            .build();
        assert!(config.file.enabled);
        assert_eq!(config.file.directory, "./test_logs");
        assert_eq!(config.file.filename, "test");
    }
}
```

#### 测试 Config

```rust
#[test]
fn test_config_serialization() {
    let config = LoggerConfig::default();
    let toml = toml::to_string(&config).unwrap();
    assert!(toml.contains("level = \"info\""));
}

#[test]
fn test_config_deserialization() {
    let toml = r#"
        level = "debug"
        [console]
        enabled = true
    "#;
    let config: LoggerConfig = toml::from_str(toml).unwrap();
    assert_eq!(config.level, LogLevel::Debug);
}
```

#### 测试错误类型

```rust
#[test]
fn test_error_display() {
    let err = LoggerError::invalid("test error");
    assert_eq!(err.to_string(), "invalid configuration: test error");
}

#[test]
fn test_error_from_io() {
    let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
    let err: LoggerError = io_err.into();
    assert!(matches!(err, LoggerError::Io(_)));
}
```

---

## 集成测试

### 测试位置

集成测试位于 `tests/` 目录：

```
tests/
├── builder_test.rs
├── console_test.rs
├── file_test.rs
└── common/
    └── mod.rs  ← 共享测试工具
```

### 测试初始化

```rust
// tests/builder_test.rs

use infinity_logger::{Logger, config::LogLevel};

#[test]
fn test_default_init() {
    let result = Logger::builder().init();
    // 注意：tracing subscriber 只能初始化一次
    // 在测试中需要特殊处理
    assert!(result.is_ok() || matches!(result, Err(LoggerError::AlreadyInitialized)));
}
```

### 测试文件输出

```rust
// tests/file_test.rs

use std::fs;
use std::path::Path;
use infinity_logger::{Logger, config::Rotation};

#[test]
fn test_file_creation() {
    let test_dir = "./test_logs";
    let test_file = "test_app";

    // 清理测试目录
    let _ = fs::remove_dir_all(test_dir);

    // 初始化 Logger
    Logger::builder()
        .file(true)
        .directory(test_dir)
        .filename(test_file)
        .init()
        .ok(); // 忽略 AlreadyInitialized 错误

    // 写入日志
    tracing::info!("Test log message");

    // 验证文件存在
    assert!(Path::new(test_dir).exists());

    // 清理
    let _ = fs::remove_dir_all(test_dir);
}
```

### 测试 JSON 输出

```rust
// tests/json_test.rs

#[test]
#[cfg(feature = "json")]
fn test_json_format() {
    Logger::builder()
        .json(true)
        .init()
        .ok();

    // JSON 输出会包含结构化字段
    tracing::info!(user_id = 123, "Test message");
    // 需要捕获输出进行验证（复杂，通常手动测试）
}
```

### 共享测试工具

```rust
// tests/common/mod.rs

use std::sync::Once;

static INIT: Once = Once::new();

/// 确保 Logger 只初始化一次
pub fn init_test_logger() {
    INIT.call_once(|| {
        infinity_logger::Logger::builder()
            .init()
            .ok();
    });
}
```

使用：

```rust
// tests/integration_test.rs

mod common;

#[test]
fn test_something() {
    common::init_test_logger();
    tracing::info!("Test log");
}
```

---

## 示例测试

### 示例结构

```rust
// examples/basic.rs

use infinity_logger::Logger;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    Logger::builder().init()?;
    tracing::info!("Hello, Infinity!");
    Ok(())
}
```

### 运行示例

```bash
# 运行单个示例
cargo run --example basic

# 运行所有示例
cargo run --examples

# 带 feature
cargo run --example file --features file
```

### 示例测试脚本

```bash
#!/bin/bash
# scripts/test_examples.sh

set -e

echo "Testing examples..."

cargo run --example basic
cargo run --example file --features file
cargo run --example json --features json

echo "All examples passed!"
```

---

## 文档测试

### RustDoc 测试

```rust
/// 初始化 Logger
///
/// # Examples
///
/// ```
/// use infinity_logger::Logger;
///
/// # fn main() -> Result<(), infinity_logger::LoggerError> {
/// Logger::builder().init()?;
/// tracing::info!("Hello");
/// # Ok(())
/// # }
/// ```
pub fn init() -> Result<()> {
    Logger::builder().init()
}
```

### 运行文档测试

```bash
# 运行所有文档测试
cargo test --doc

# 显示输出
cargo test --doc -- --nocapture
```

### 隐藏辅助代码

```rust
/// ```
/// # use infinity_logger::Logger;
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// Logger::builder().init()?;
/// # Ok(())
/// # }
/// ```
```

`#` 开头的行在文档中隐藏，但会执行。

---

## 性能测试

### Benchmark 结构

```rust
// benches/logger_bench.rs

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use infinity_logger::Logger;

fn bench_init(c: &mut Criterion) {
    c.bench_function("logger_init", |b| {
        b.iter(|| {
            // 注意：实际只能初始化一次
            Logger::builder().build()
        });
    });
}

criterion_group!(benches, bench_init);
criterion_main!(benches);
```

### 添加依赖

```toml
[dev-dependencies]
criterion = "0.5"

[[bench]]
name = "logger_bench"
harness = false
```

### 运行 Benchmark

```bash
cargo bench
```

---

## 测试工具

### 1. 捕获日志输出

使用 `tracing-subscriber` 的测试工具：

```rust
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

#[test]
fn test_log_capture() {
    let subscriber = tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer());
    
    let _guard = tracing::subscriber::set_default(subscriber);
    
    tracing::info!("Test message");
}
```

### 2. 临时目录

```rust
use tempfile::TempDir;

#[test]
fn test_with_temp_dir() {
    let temp_dir = TempDir::new().unwrap();
    let log_path = temp_dir.path().join("logs");

    Logger::builder()
        .file(true)
        .directory(log_path.to_str().unwrap())
        .init()
        .ok();

    // temp_dir 在 Drop 时自动清理
}
```

添加依赖：

```toml
[dev-dependencies]
tempfile = "3"
```

### 3. 异步测试

```rust
#[tokio::test]
async fn test_async_logging() {
    Logger::builder().init().ok();
    
    tokio::spawn(async {
        tracing::info!("Async log");
    }).await.unwrap();
}
```

---

## CI/CD

### GitHub Actions 配置

```yaml
# .github/workflows/test.yml

name: Tests

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    
    steps:
      - uses: actions/checkout@v3
      
      - name: Setup Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          override: true
      
      - name: Run tests
        run: cargo test --all-features
      
      - name: Run clippy
        run: cargo clippy --all-features -- -D warnings
      
      - name: Check formatting
        run: cargo fmt -- --check
      
      - name: Build docs
        run: cargo doc --no-deps --all-features
```

### 测试矩阵

```yaml
strategy:
  matrix:
    os: [ubuntu-latest, macos-latest, windows-latest]
    rust: [stable, beta, nightly]
```

### 本地 CI 脚本

```bash
#!/bin/bash
# scripts/ci.sh

set -e

echo "Running CI checks..."

echo "1. Format check..."
cargo fmt -- --check

echo "2. Clippy..."
cargo clippy --all-features -- -D warnings

echo "3. Tests..."
cargo test --all-features

echo "4. Doc tests..."
cargo test --doc

echo "5. Examples..."
cargo build --examples --all-features

echo "6. Documentation..."
cargo doc --no-deps --all-features

echo "All checks passed!"
```

---

## 测试覆盖率

### 使用 tarpaulin

```bash
# 安装
cargo install cargo-tarpaulin

# 运行
cargo tarpaulin --all-features --out Html
```

### 目标覆盖率

| 模块       | 目标覆盖率 |
|------------|-----------|
| builder    | ≥ 95%     |
| config     | ≥ 95%     |
| error      | ≥ 90%     |
| layer      | ≥ 85%     |
| middleware | ≥ 80%     |
| util       | ≥ 90%     |

---

## 测试最佳实践

### 1. 命名规范

```rust
#[test]
fn test_<function>_<scenario>_<expected>() {
    // 例如：
    // test_builder_with_invalid_directory_returns_error
}
```

### 2. AAA 模式

```rust
#[test]
fn test_example() {
    // Arrange - 准备
    let config = LoggerConfig::default();
    
    // Act - 执行
    let level = config.level;
    
    // Assert - 断言
    assert_eq!(level, LogLevel::Info);
}
```

### 3. 避免测试间依赖

```rust
// ❌ 错误：依赖全局状态
#[test]
fn test_a() {
    Logger::builder().init().unwrap();
}

#[test]
fn test_b() {
    // 失败，因为 Logger 已初始化
    Logger::builder().init().unwrap();
}

// ✅ 正确：使用独立配置
#[test]
fn test_a() {
    let config = LoggerBuilder::new().build();
    assert_eq!(config.level, LogLevel::Info);
}
```

### 4. 清理测试资源

```rust
#[test]
fn test_with_cleanup() {
    let test_dir = "./test_logs";
    
    // 测试逻辑
    
    // 清理
    let _ = std::fs::remove_dir_all(test_dir);
}
```

---

## 故障排查

### 测试失败：AlreadyInitialized

**原因**：`tracing` subscriber 只能初始化一次

**解决**：
```rust
Logger::builder().init().ok(); // 忽略错误
// 或
if let Err(e) = Logger::builder().init() {
    if !matches!(e, LoggerError::AlreadyInitialized) {
        panic!("Init failed: {}", e);
    }
}
```

### 测试间干扰

**原因**：共享全局状态

**解决**：使用 `serial_test`：

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

#[test]
#[serial]
fn test_b() {
    // 顺序执行，不会冲突
}
```

---

## 下一步

- [BENCHMARK.md](BENCHMARK.md) - 性能基准测试
- [CONTRIBUTING.md](CONTRIBUTING.md) - 贡献测试用例
- [CODING_STANDARD.md](CODING_STANDARD.md) - 测试代码规范

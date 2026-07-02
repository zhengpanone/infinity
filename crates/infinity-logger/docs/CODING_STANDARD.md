# Infinity Workspace Coding Standard

> **Project:** Infinity Workspace
> **Applies To:** All Crates (`infinity-*`)
> **Version:** v1.0
> **Status:** Frozen

---

# 1. Purpose

本文档定义 Infinity Workspace 的统一编码规范。

目标：

* 保持所有 crate 风格一致
* 提高代码可读性
* 降低维护成本
* 保持 API 一致性
* 提高可测试性
* 支持长期演进

所有新模块必须遵循本规范。

---

# 2. General Principles

遵循以下原则：

* Official First
* Simplicity First
* Readability First
* Composition over Inheritance
* Zero-cost Abstraction
* Backward Compatibility
* Documentation First

---

# 3. Project Layout

所有 crate 保持统一目录：

```text
crate-name
│
├── Cargo.toml
├── README.md
├── CHANGELOG.md
├── LICENSE
│
├── examples/
├── benches/
├── tests/
├── docs/
│
└── src/
    ├── lib.rs
    ├── error.rs
    ├── config.rs
    ├── builder.rs
    ├── util/
    └── ...
```

---

# 4. Module Rules

每个模块只有一个职责。

例如：

```text
builder.rs
```

仅负责：

* Builder API

不得：

* 初始化资源
* IO
* 网络操作

---

```text
config.rs
```

仅负责：

* 数据结构
* 默认配置
* 配置校验

---

```text
init.rs
```

负责：

* 初始化
* 资源创建

---

禁止：

模块之间循环依赖。

---

# 5. Public API Design

所有公开 API：

* 简洁
* 稳定
* 一致

例如：

```rust
LoggerBuilder::default()
    .level(LogLevel::Info)
    .init()?;
```

避免：

```rust
Logger::new()
```

```rust
Logger::create()
```

```rust
Logger::start()
```

统一使用：

```rust
init()
```

---

# 6. Builder Convention

Builder：

* 消费 `self`
* 返回 `Self`
* 最终 `build()` 或 `init()`

例如：

```rust
pub fn level(
    mut self,
    level: LogLevel,
) -> Self
```

禁止：

```rust
&mut Self
```

保持 Builder 链式风格一致。

---

# 7. Error Handling

统一：

```rust
pub type Result<T> =
    std::result::Result<T, Error>;
```

所有 crate：

```rust
crate::Result<T>
```

禁止公开：

```rust
anyhow::Result
```

禁止：

```rust
Box<dyn Error>
```

---

统一错误类型：

```text
crate::Error
```

例如：

```text
LoggerError
```

---

# 8. Panic Policy

业务代码禁止：

```rust
unwrap()

expect()

panic!()

todo!()

unreachable!()
```

允许：

测试代码。

---

# 9. Naming Convention

类型：

```text
PascalCase
```

例如：

```text
LoggerBuilder
```

---

函数：

```text
snake_case
```

例如：

```text
build_layer
```

---

模块：

```text
snake_case
```

例如：

```text
rolling_file
```

---

常量：

```text
UPPER_SNAKE_CASE
```

---

Trait：

使用名词：

```text
Formatter

Encoder

Exporter
```

避免：

```text
IFormatter
```

---

# 10. File Organization

每个 Rust 文件：

```rust
//! Module Documentation

use ...

pub struct

impl

trait

fn

tests
```

保持一致。

---

# 11. Imports

顺序：

```rust
// std

// third-party

// crate
```

例如：

```rust
use std::path::PathBuf;

use tracing::Level;

use crate::config::LoggerConfig;
```

避免混合排序。

---

# 12. Documentation

所有：

```rust
pub
```

必须编写 RustDoc。

包括：

* 功能
* 参数
* 返回值
* Errors
* Examples

例如：

```rust
/// Initializes the logger.
///
/// # Errors
///
/// Returns an error if the global subscriber
/// has already been initialized.
```

---

# 13. Testing

每个模块：

至少：

* Unit Test

公开 API：

必须：

* Integration Test

Examples：

必须：

可运行。

---

# 14. Benchmark

性能敏感模块：

增加：

```text
benches/
```

例如：

* Formatter
* Writer
* Filter

---

# 15. Logging

库内部：

统一：

```rust
tracing
```

禁止：

```rust
println!
```

生产代码禁止：

```rust
dbg!
```

---

# 16. Async Rules

异步 API：

必须：

```rust
Send + Sync
```

避免：

阻塞操作。

文件 IO：

采用：

异步或 NonBlocking。

---

# 17. Unsafe Code

原则：

禁止使用 `unsafe`。

如果确实需要：

必须：

* 注释原因
* 安全性说明
* 单元测试

并通过 Code Review。

---

# 18. Cargo Features

所有可选功能：

采用：

```toml
default = []

feature = []
```

不得隐藏行为。

Feature 必须相互独立。

---

# 19. Dependency Management

原则：

* 优先标准库
* 优先 Rust 官方生态
* 最小依赖
* 避免重复功能依赖

新增依赖需说明原因。

---

# 20. Performance Guidelines

原则：

* 避免不必要分配
* 避免 Clone
* 优先借用
* 零成本抽象
* 热路径避免锁竞争

性能优化必须基于测试数据，而非猜测。

---

# 21. Security Guidelines

不得：

* 输出敏感信息
* 记录密码
* 记录 Token
* 记录密钥

日志涉及用户信息时，应提供脱敏能力。

---

# 22. Compatibility

所有公开 API：

遵循 SemVer。

Breaking Change：

必须：

* 更新文档
* 更新 CHANGELOG
* 升级 Major 版本

---

# 23. CI Requirements

每次提交必须通过：

```bash
cargo fmt --check

cargo check

cargo clippy --all-features -- -D warnings

cargo test

cargo doc --no-deps
```

CI 未通过禁止合并。

---

# 24. Code Review Checklist

提交前确认：

* [ ] API 是否简洁？
* [ ] 是否符合单一职责？
* [ ] 是否避免 panic？
* [ ] 是否补充文档？
* [ ] 是否包含测试？
* [ ] 是否保持向后兼容？
* [ ] 是否引入不必要依赖？
* [ ] 是否符合本规范？

---

# 25. Workspace Conventions

所有 `infinity-*` crate：

统一遵循：

* 相同目录结构
* 相同 Builder 风格
* 相同 Error 模型
* 相同 Result 类型
* 相同 Feature 设计
* 相同测试规范
* 相同文档风格

确保整个 Workspace 拥有一致的开发体验。

---

# 26. Revision History

| Version | Date       | Description                             |
| ------- | ---------- | --------------------------------------- |
| 1.0     | 2026-07-03 | 初始编码规范。适用于 Infinity Workspace 全部 crate。 |

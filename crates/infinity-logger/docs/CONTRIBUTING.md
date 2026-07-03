# Contributing Guide

> **Project:** infinity-logger  
> **Version:** v1.0

欢迎为 Infinity Logger 做出贡献！本文档将帮助你了解如何参与项目开发。

---

## 目录

- [行为准则](#行为准则)
- [如何贡献](#如何贡献)
- [开发环境](#开发环境)
- [代码规范](#代码规范)
- [提交规范](#提交规范)
- [Pull Request 流程](#pull-request-流程)
- [测试要求](#测试要求)
- [文档要求](#文档要求)

---

## 行为准则

参与本项目，你需要遵守以下准则：

- **尊重他人** - 尊重所有贡献者和用户
- **建设性反馈** - 提供有价值的、建设性的意见
- **协作精神** - 以开放和友好的态度合作
- **专业态度** - 保持专业和礼貌的交流

---

## 如何贡献

### 报告 Bug

发现 Bug？请提交 Issue，包含：

1. **描述** - 清楚描述问题
2. **重现步骤** - 详细的重现步骤
3. **预期行为** - 你期望的结果
4. **实际行为** - 实际发生的情况
5. **环境信息** - Rust 版本、操作系统等

**Issue 模板**：

```markdown
**描述**
简洁描述 Bug

**重现步骤**
1. 初始化 Logger
2. 调用 xxx
3. 观察错误

**预期行为**
应该输出日志

**实际行为**
抛出异常：xxx

**环境**
- Rust 版本: 1.88
- OS: Ubuntu 22.04
- infinity-logger 版本: 0.1.0
- Features: ["file", "json"]
```

### 功能建议

提交功能建议时，请说明：

1. **用例** - 为什么需要这个功能
2. **设计方案** - 你的初步设计想法
3. **替代方案** - 是否有其他实现方式
4. **影响范围** - 对现有 API 的影响

### 改进文档

文档改进包括：

- 修正拼写或语法错误
- 添加示例
- 完善说明
- 翻译文档

直接提交 PR 即可。

### 代码贡献

1. Fork 本仓库
2. 创建特性分支
3. 编写代码
4. 添加测试
5. 提交 PR

---

## 开发环境

### 环境要求

- **Rust**: ≥ 1.88 (MSRV)
- **操作系统**: Linux / macOS / Windows
- **编辑器**: VS Code / CLion / RustRover（推荐）

### 克隆仓库

```bash
git clone https://github.com/zhengpanone/infinity.git
cd infinity/crates/infinity-logger
```

### 安装依赖

```bash
# 检查 Rust 版本
rustc --version

# 更新工具链
rustup update

# 安装 clippy 和 rustfmt
rustup component add clippy rustfmt
```

### 构建项目

```bash
# 检查语法
cargo check

# 构建
cargo build

# 构建所有 features
cargo build --all-features
```

### 运行测试

```bash
# 运行测试
cargo test

# 所有 features
cargo test --all-features

# 显示输出
cargo test -- --nocapture
```

### 代码检查

```bash
# 格式化
cargo fmt

# Clippy 检查
cargo clippy --all-features -- -D warnings

# 生成文档
cargo doc --no-deps --all-features --open
```

---

## 代码规范

### Rust 代码风格

遵循 [Rust 官方风格指南](https://doc.rust-lang.org/nightly/style-guide/)：

- 使用 `cargo fmt` 格式化代码
- 通过 `cargo clippy` 检查
- 避免 `unwrap()` 和 `expect()`（测试代码除外）
- 使用 `Result<T>` 处理错误

### 命名规范

```rust
// 类型：PascalCase
pub struct LoggerConfig { }
pub enum LogLevel { }

// 函数和变量：snake_case
pub fn init_logger() { }
let log_level = LogLevel::Info;

// 常量：SCREAMING_SNAKE_CASE
pub const MAX_FILE_SIZE: usize = 1024;

// 生命周期：小写单字母
fn example<'a>(s: &'a str) { }
```

### 模块组织

```rust
// 导入顺序
use std::io;                    // 标准库
use tracing::info;              // 外部 crate
use crate::config::LogLevel;    // 当前 crate
```

### 错误处理

```rust
// ✅ 正确：使用 Result
pub fn init() -> Result<()> {
    // ...
    Ok(())
}

// ❌ 错误：使用 panic
pub fn init() {
    // ...
    panic!("Failed to init");
}
```

### 文档注释

```rust
/// 初始化日志系统
///
/// 使用默认配置初始化 `tracing` subscriber。
///
/// # Errors
///
/// 如果 subscriber 已经初始化，返回 [`LoggerError::AlreadyInitialized`]。
///
/// # Examples
///
/// ```
/// use infinity_logger::Logger;
///
/// # fn main() -> Result<(), infinity_logger::LoggerError> {
/// Logger::builder().init()?;
/// # Ok(())
/// # }
/// ```
pub fn init() -> Result<()> {
    // ...
}
```

---

## 提交规范

### Commit Message 格式

使用 [Conventional Commits](https://www.conventionalcommits.org/) 规范：

```
<type>(<scope>): <subject>

<body>

<footer>
```

### Type 类型

- `feat`: 新功能
- `fix`: Bug 修复
- `docs`: 文档更新
- `style`: 代码格式（不影响功能）
- `refactor`: 重构
- `test`: 测试相关
- `chore`: 构建/工具相关

### Scope 范围

- `builder`: Builder 模块
- `config`: 配置模块
- `layer`: Layer 模块
- `middleware`: 中间件
- `docs`: 文档
- `ci`: CI/CD

### 示例

```bash
# 新功能
git commit -m "feat(layer): add JSON formatter support"

# Bug 修复
git commit -m "fix(builder): validate directory before init"

# 文档
git commit -m "docs(readme): update installation instructions"

# 重构
git commit -m "refactor(config): simplify LogLevel enum"

# 测试
git commit -m "test(builder): add unit tests for file config"
```

### 详细示例

```
feat(layer): add JSON formatter support

Implement JSON output layer for structured logging.

- Add JsonConfig struct
- Implement json::layer() function
- Add feature gate "json"
- Update documentation

Closes #123
```

---

## Pull Request 流程

### 1. 创建分支

```bash
# 从 main 创建特性分支
git checkout -b feat/json-formatter

# 或修复分支
git checkout -b fix/builder-validation
```

### 2. 开发与提交

```bash
# 编写代码
# ...

# 运行测试
cargo test --all-features

# 提交
git add .
git commit -m "feat(layer): add JSON formatter"
```

### 3. 推送分支

```bash
git push origin feat/json-formatter
```

### 4. 创建 Pull Request

在 GitHub 上创建 PR，填写：

**标题**：
```
feat(layer): add JSON formatter support
```

**描述**：
```markdown
## 变更内容

添加 JSON 格式化支持

## 动机

支持结构化日志输出，便于日志分析系统解析

## 实现细节

- 新增 `JsonConfig` 配置结构
- 实现 `json::layer()` 函数
- 添加 `json` feature gate
- 添加单元测试和集成测试

## 测试

- [ ] 单元测试通过
- [ ] 集成测试通过
- [ ] 示例代码运行正常

## 相关 Issue

Closes #123
```

### 5. Code Review

- 回应 Review 意见
- 根据反馈修改代码
- 推送更新

```bash
# 修改代码后
git add .
git commit -m "fix: address review comments"
git push
```

### 6. 合并

PR 通过审核后，维护者会合并到主分支。

---

## 测试要求

### 单元测试

每个新功能必须包含单元测试：

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_feature() {
        // Arrange
        let config = LoggerConfig::default();
        
        // Act
        let result = some_function(&config);
        
        // Assert
        assert!(result.is_ok());
    }
}
```

### 集成测试

复杂功能需要集成测试：

```rust
// tests/new_feature_test.rs

use infinity_logger::Logger;

#[test]
fn test_feature_integration() {
    Logger::builder()
        .new_feature(true)
        .init()
        .ok();
    
    // 验证功能正常工作
}
```

### 测试覆盖率

- 核心模块覆盖率 ≥ 90%
- 新功能覆盖率 ≥ 85%

运行：

```bash
cargo tarpaulin --all-features
```

---

## 文档要求

### RustDoc

所有公开 API 必须有文档注释：

```rust
/// 日志配置结构
///
/// 包含所有日志系统的配置项。
///
/// # Examples
///
/// ```
/// use infinity_logger::config::LoggerConfig;
///
/// let config = LoggerConfig::default();
/// ```
pub struct LoggerConfig {
    // ...
}
```

### README 更新

新功能需要更新 README.md：

- 在 Features 表格中添加
- 添加使用示例
- 更新 Quick Start（如需要）

### 示例代码

新功能建议添加示例：

```rust
// examples/new_feature.rs

use infinity_logger::Logger;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    Logger::builder()
        .new_feature(true)
        .init()?;

    tracing::info!("Demo message");
    Ok(())
}
```

### CHANGELOG

重要变更需要更新 CHANGELOG.md：

```markdown
## [Unreleased]

### Added
- JSON formatter support (#123)

### Fixed
- Builder validation bug (#124)
```

---

## CI 检查清单

PR 必须通过所有 CI 检查：

```bash
# 1. 格式检查
cargo fmt -- --check

# 2. Clippy 检查
cargo clippy --all-features -- -D warnings

# 3. 测试
cargo test --all-features

# 4. 文档测试
cargo test --doc

# 5. 构建文档
cargo doc --no-deps --all-features

# 6. 示例构建
cargo build --examples --all-features
```

本地运行完整 CI：

```bash
./scripts/ci.sh
```

---

## 发布流程

（仅限维护者）

### 1. 版本更新

更新 `Cargo.toml`：

```toml
[package]
version = "0.2.0"
```

### 2. 更新 CHANGELOG

```markdown
## [0.2.0] - 2026-07-10

### Added
- JSON formatter support
- File rotation

### Fixed
- Builder validation
```

### 3. 创建 Tag

```bash
git tag -a v0.2.0 -m "Release v0.2.0"
git push origin v0.2.0
```

### 4. 发布到 crates.io

```bash
cargo publish
```

---

## 获取帮助

遇到问题？

- 📖 查看 [文档](https://docs.rs/infinity-logger)
- 💬 提交 [Issue](https://github.com/zhengpanone/infinity/issues)
- 📧 联系维护者

---

## 致谢

感谢所有贡献者！

你的贡献让 Infinity Logger 变得更好。🎉

# Release Process

> **Project:** infinity-logger  
> **Version:** v1.0  
> **Status:** Active

本文档描述 Infinity Logger 的发布流程。

---

## 目录

- [发布策略](#发布策略)
- [版本号规则](#版本号规则)
- [发布检查清单](#发布检查清单)
- [发布流程](#发布流程)
- [回滚流程](#回滚流程)

---

## 发布策略

### 版本规划

- **0.x** - 快速迭代，API 可变动
- **1.0** - 稳定版本，API 冻结
- **1.x** - 新功能，保持向后兼容
- **2.x** - Breaking Changes

### 发布频率

- **Patch (0.1.x)** - Bug 修复，按需发布
- **Minor (0.x.0)** - 新功能，每月 1-2 次
- **Major (x.0.0)** - 重大变更，每年 1-2 次

---

## 版本号规则

遵循 [Semantic Versioning 2.0.0](https://semver.org/)：

```
MAJOR.MINOR.PATCH
```

### Major (重大变更)

触发条件：
- 删除公开 API
- 修改公开 API 签名
- 修改默认行为
- 不兼容的配置变更

示例：
```
0.9.5 → 1.0.0
1.5.3 → 2.0.0
```

### Minor (新功能)

触发条件：
- 添加新功能
- 添加新 API
- 新增 Cargo feature
- 弃用（但未删除）API

示例：
```
0.1.0 → 0.2.0
1.0.0 → 1.1.0
```

### Patch (修复)

触发条件：
- Bug 修复
- 文档更新
- 性能优化（不改变 API）
- 依赖版本更新（兼容）

示例：
```
0.1.0 → 0.1.1
1.0.0 → 1.0.1
```

---

## 发布检查清单

### 1. 代码检查

- [ ] 所有测试通过
- [ ] Clippy 无警告
- [ ] Rustfmt 格式化
- [ ] 文档生成成功
- [ ] 示例可运行

```bash
cargo test --all-features
cargo clippy --all-features -- -D warnings
cargo fmt -- --check
cargo doc --no-deps --all-features
cargo run --example basic
```

### 2. 版本更新

- [ ] 更新 Cargo.toml 版本号
- [ ] 更新 CHANGELOG.md
- [ ] 更新 README.md（如需要）
- [ ] 更新文档中的版本引用

### 3. 文档检查

- [ ] API 文档完整
- [ ] 示例代码正确
- [ ] CHANGELOG 包含所有变更
- [ ] Migration Guide（重大变更）

### 4. 依赖检查

- [ ] 依赖版本兼容
- [ ] 无安全漏洞

```bash
cargo audit
cargo outdated
```

### 5. 兼容性测试

- [ ] MSRV 测试（Rust 1.88+）
- [ ] 多平台测试（Linux/macOS/Windows）
- [ ] 所有 feature 组合测试

---

## 发布流程

### Step 1: 准备发布分支

```bash
git checkout main
git pull origin main
git checkout -b release/v0.2.0
```

### Step 2: 更新版本号

编辑 Cargo.toml:

```toml
[package]
version = "0.2.0"
```

### Step 3: 更新 CHANGELOG

```markdown
## [0.2.0] - 2026-07-10

### Added
- File rotation support
- JSON formatter

### Fixed
- Builder validation bug

### Changed
- Improved error messages
```

### Step 4: 提交变更

```bash
git add Cargo.toml CHANGELOG.md
git commit -m "chore(release): prepare v0.2.0"
git push origin release/v0.2.0
```

### Step 5: 创建 PR 并合并

### Step 6: 创建 Git Tag

```bash
git tag -a v0.2.0 -m "Release v0.2.0"
git push origin v0.2.0
```

### Step 7: 发布到 crates.io

```bash
cargo publish --dry-run
cargo publish
```

### Step 8: 创建 GitHub Release

---

## 回滚流程

```bash
# Yank 版本
cargo yank --vers 0.2.0

# 取消 yank
cargo yank --vers 0.2.0 --undo
```

---

**记住：发布前三思，发布后负责。**

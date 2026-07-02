# Infinity Workspace API Guidelines

> **Project:** Infinity Workspace
> **Version:** v1.0
> **Status:** Frozen
> **Applies To:** All `infinity-*` crates

---

# 1. Purpose

本文档定义 Infinity Workspace 的公共 API 设计规范。

目标：

* 保持所有 crate API 风格一致
* 提供稳定、易理解、易扩展的接口
* 保证长期向后兼容
* 降低学习成本

所有对外公开 API 必须遵循本规范。

---

# 2. API Design Principles

所有公开 API 应遵循以下原则：

* Minimal（最小）
* Consistent（一致）
* Predictable（可预测）
* Explicit（明确）
* Backward Compatible（向后兼容）

避免：

* 过度封装
* 隐式行为
* 重复接口
* 命名歧义

---

# 3. Public API Surface

默认情况下：

只有真正需要暴露的内容才允许 `pub`。

例如：

```rust
pub struct LoggerBuilder;

pub struct LoggerConfig;

pub enum LogLevel;

pub enum Rotation;
```

内部实现：

```rust
pub(crate)
```

禁止：

```rust
pub mod internal;
```

---

# 4. Module Design

推荐：

```text
crate
│
├── builder.rs
├── config.rs
├── error.rs
├── util/
└── ...
```

不要：

```text
helper.rs

common.rs

utils.rs
```

如果模块过大：

继续拆分。

---

# 5. Builder Pattern

Builder 是默认模式。

例如：

```rust
LoggerBuilder::default()
```

Builder：

* 消费 self
* 返回 Self

例如：

```rust
pub fn level(
    mut self,
    level: LogLevel,
) -> Self
```

最终：

```rust
.init()

.build()
```

结束。

禁止：

```rust
&mut self
```

链式 Builder。

---

# 6. Constructor Guidelines

推荐：

```rust
Type::new(...)
```

如果存在默认值：

```rust
Type::default()
```

如果来自配置：

```rust
Type::from_config(...)
```

如果来自文件：

```rust
Type::from_file(...)
```

如果来自字符串：

```rust
Type::from_str(...)
```

不要：

```rust
create()

make()

build_new()

init_new()
```

---

# 7. Method Naming

统一：

| Action | Method        |
| ------ | ------------- |
| 创建     | new           |
| 默认     | default       |
| 配置     | from_config   |
| 文件     | from_file     |
| 初始化    | init          |
| 构建     | build         |
| 启动     | start（仅运行时组件） |
| 停止     | shutdown      |
| 重载     | reload        |
| 获取     | get_*         |
| 设置     | set_*（仅可变对象）  |

避免：

```text
run

execute

process

do
```

除非语义明确。

---

# 8. Parameter Ordering

参数顺序统一：

```text
Required
↓

Configuration

↓

Optional
```

例如：

```rust
new(path, options)
```

不要：

```rust
new(options, path)
```

Builder 不允许超过一个必填参数。

---

# 9. Return Types

统一：

```rust
Result<T>
```

禁止公开：

```rust
std::result::Result
```

统一：

```rust
crate::Result<T>
```

---

# 10. Error Design

每个 crate：

只有一个 Error 类型。

例如：

```rust
LoggerError
```

所有错误：

转换：

```rust
From
```

实现。

禁止：

```rust
Box<dyn Error>
```

作为 Public API。

---

# 11. Trait Design

Trait：

采用能力命名。

例如：

```rust
Formatter

Exporter

Loader
```

不要：

```text
IFormatter
```

Trait 尽量：

* 小
* 单一职责

---

# 12. Generic Design

优先：

```rust
impl Trait
```

其次：

泛型。

最后：

Trait Object。

不要为了泛型而泛型。

---

# 13. Async API

如果 API：

涉及：

* IO
* Network
* Database

采用：

```rust
async fn
```

CPU：

保持同步。

不要：

同步 API 包装异步。

---

# 14. Configuration

所有配置：

采用：

```rust
Config
```

例如：

```rust
LoggerConfig

DatabaseConfig
```

Builder：

只修改 Config。

---

# 15. Visibility Rules

默认：

```rust
private
```

其次：

```rust
pub(crate)
```

最后：

```rust
pub
```

尽量缩小可见性范围。

---

# 16. Feature Flags

所有可选功能：

必须：

Cargo Feature。

禁止：

运行时判断是否存在依赖。

例如：

```toml
json

file

otel
```

---

# 17. Documentation

所有：

```rust
pub
```

必须：

RustDoc。

包括：

* Summary
* Arguments
* Returns
* Errors
* Example

---

# 18. Deprecation Policy

弃用：

采用：

```rust
#[deprecated]
```

保留：

至少一个 Minor 版本。

不要：

直接删除 Public API。

---

# 19. Compatibility

遵循：

Semantic Versioning。

Breaking Change：

必须：

Major。

新增 API：

Minor。

Bug Fix：

Patch。

---

# 20. Examples

所有公开 API：

必须：

至少一个可运行示例。

Examples：

必须：

纳入 CI。

---

# 21. Testing Requirements

每个公开 API：

至少：

* Unit Test
* Integration Test（关键路径）

重要示例：

应作为回归测试的一部分。

---

# 22. Performance Guidelines

API 设计应避免：

* 不必要的 Clone
* 多余分配
* 隐式阻塞
* 热路径锁竞争

性能优化以基准测试结果为依据。

---

# 23. Security Guidelines

公共 API：

不得：

* 默认输出敏感信息
* 暴露内部状态
* 泄露凭据

涉及日志、配置等模块时，应支持敏感字段脱敏。

---

# 24. API Review Checklist

新增 Public API 前：

* [ ] 是否已有类似接口？
* [ ] 是否符合命名规范？
* [ ] 是否符合 Builder 风格？
* [ ] 是否返回 `crate::Result<T>`？
* [ ] 是否编写 RustDoc？
* [ ] 是否提供 Example？
* [ ] 是否添加测试？
* [ ] 是否考虑向后兼容？

---

# 25. API Evolution

Public API 一经发布：

优先：

* 新增能力
* 扩展配置
* 增加 Feature

避免：

* 修改已有方法签名
* 修改默认行为
* 删除已有接口

所有重大变更必须：

1. 更新 DESIGN.md
2. 更新 ARCHITECTURE.md
3. 更新 CHANGELOG.md
4. 评估兼容性影响

---

# 26. Summary

Infinity Workspace 的 API 设计遵循：

* 一致优于灵活
* 明确优于隐式
* 组合优于继承
* 稳定优于频繁变化
* 简单优于复杂

所有 `infinity-*` crate 应共享相同的 API 风格，为使用者提供统一、稳定、可预测的开发体验。

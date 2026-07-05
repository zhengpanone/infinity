# Infinity Error

Infinity 工作区共享错误基础设施。

`infinity-error` 提供一套轻量、稳定的错误封装，用于统一应用层和基础设施层的错误分类。它不绑定 `sqlx`、Redis、Axum、Tonic 等具体依赖，而是让各个 crate 在自己的边界处把第三方错误映射成工作区统一错误。

## 特性

- `InfinityError`：工作区共享错误枚举
- `Result<T>`：统一结果类型别名
- `ErrorKind`：稳定、可匹配的错误分类（含 `ALL` / `from_code` 反查）
- `ResultExt`：在 crate 边界一行把第三方 `Result` 转换为工作区错误，并保留类型化来源链
- `bail!` / `ensure!`：以指定分类快速返回错误的宏
- 默认 HTTP 状态码映射，方便后续 Web/API 层使用
- 低依赖设计，避免基础设施 crate 之间产生循环依赖

## 快速开始

```rust
use infinity_error::{InfinityError, Result};

fn load_user(id: &str) -> Result<()> {
    if id.trim().is_empty() {
        return Err(InfinityError::validation_field("id", "must not be empty"));
    }

    Err(InfinityError::not_found(format!("user:{id}")))
}
```

## 错误分类

| 分类 | 构造函数 | 默认 HTTP 状态码 |
| ---- | -------- | ---------------- |
| `config` | `InfinityError::config(...)` | 500 |
| `logger` | `InfinityError::logger(...)` | 500 |
| `database` | `InfinityError::database(...)` | 500 |
| `cache` | `InfinityError::cache(...)` | 500 |
| `auth` | `InfinityError::auth(...)` | 500 |
| `web` | `InfinityError::web(...)` | 500 |
| `validation` | `InfinityError::validation(...)` | 400 |
| `not_found` | `InfinityError::not_found(...)` | 404 |
| `conflict` | `InfinityError::conflict(...)` | 409 |
| `unsupported` | `InfinityError::unsupported(...)` | 501 |

## 集成方式

基础设施 crate 应该在自己的边界处转换第三方错误。最省事的方式是用 `ResultExt`，它会**保留底层错误的类型化来源**（`source()` 可访问完整错误链）：

```rust
use infinity_error::{ErrorKind, Result, ResultExt};

fn read_port(raw: &str) -> Result<u16> {
    // Display 为「解析端口」，底层 ParseIntError 通过 source() 保留
    raw.parse::<u16>().context(ErrorKind::Parse, "解析端口")
}
```

需要惰性构造消息（例如包含格式化开销）时用 `with_context`，闭包只在出错时求值：

```rust
use infinity_error::{ErrorKind, Result, ResultExt};

fn query(sql: &str) -> Result<()> {
    do_query(sql).with_context(ErrorKind::Database, || format!("执行查询: {sql}"))
}
# fn do_query(_: &str) -> Result<()> { Ok(()) }
```

在函数内部提前返回错误，可用 `bail!` / `ensure!` 宏：

```rust
use infinity_error::{bail, ensure, ErrorKind, Result};

fn create(name: &str, quota: i64) -> Result<()> {
    ensure!(!name.is_empty(), ErrorKind::Validation, "name 不能为空");
    if quota < 0 {
        bail!(ErrorKind::Conflict, "quota 不能为负: {quota}");
    }
    Ok(())
}
```

如果本地已经持有第三方错误，也可以直接用具体构造函数或 `with_source`：

```rust
use infinity_error::{ErrorKind, InfinityError, Result};

fn map_database_error(message: impl Into<String>) -> InfinityError {
    InfinityError::database(message)
}
```

这样可以让 `infinity-error` 保持独立，不直接依赖 `sqlx`、Redis 客户端、Axum、Tonic 或业务应用 crate。

## 开发命令

```bash
cargo fmt -p infinity-error
cargo test -p infinity-error
cargo clippy -p infinity-error --all-targets -- -D warnings
```

## 文档

- [设计说明](docs/DESIGN.md)
- [测试说明](docs/TESTING.md)
- [路线图](docs/ROADMAP.md)
- [常见问题](docs/FAQ.md)

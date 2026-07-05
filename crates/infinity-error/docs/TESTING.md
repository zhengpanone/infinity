# Infinity Error 测试说明

## 范围

`infinity-error` 是一个小型基础 crate，测试重点是 API 稳定性，而不是复杂集成行为。

## 当前覆盖

`src/lib.rs` 中的单元测试覆盖：

- 构造函数返回预期的 `ErrorKind`
- `ErrorKind::code()` 的值保持稳定
- 默认 HTTP 状态码符合设计
- 校验错误能正确展示字段路径
- 客户端错误和服务端错误分类可预期
- `std::io::Error` 会作为 source 保留
- `String` 和 `&str` 会转换为消息错误
- `InfinityError::with_kind` 覆盖每个分类，且 `Io` 会保留 source
- `ResultExt::context` / `with_context` 保留类型化来源链，`Ok` 时惰性不求值
- `with_source` 保留传入分类与底层错误链
- `ErrorKind::ALL` 与 `code()` / `from_code()` 双向 round-trip
- `bail!` / `ensure!` 宏按指定分类返回错误

## 命令

```bash
cargo fmt -p infinity-error
cargo test -p infinity-error
cargo clippy -p infinity-error --all-targets -- -D warnings
```

修改公开变体前，建议运行完整工作区测试：

```bash
cargo test --workspace
```

## 规范

- 每个新增构造函数都应补测试。
- 将 `ErrorKind::code()` 视为兼容性表面。
- 展示文本面向人类阅读，不要让下游逻辑解析展示文本。
- 下游测试优先匹配 `kind()` 或 `code()`。

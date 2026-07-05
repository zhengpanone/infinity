# Infinity Error 常见问题

## 为什么不直接到处使用 `anyhow`？

`anyhow` 很适合二进制应用和原型代码，但共享库的公开 API 需要稳定、可匹配的错误分类。`InfinityError` 提供的正是这层分类。

## 为什么这个 crate 不依赖 `sqlx`？

数据库 crate 已经依赖 `sqlx`，因此数据库相关转换应该由数据库 crate 自己负责。这样可以避免依赖循环，也能降低不使用数据库的 crate 的编译成本。

## 应用代码应该匹配展示文本吗？

不应该。展示文本面向人类和日志。程序逻辑应该使用 `err.kind()` 或 `err.code()`。

## 某个 crate 还能定义自己的错误类型吗？

可以。如果本地错误包含丰富语义，可以保留本地错误类型，并在跨工作区边界返回时转换为 `InfinityError`。

## HTTP 响应转换应该放在哪里？

应该放在 `infinity-web`。`infinity-error` 只提供默认的 `status_code()` 映射。

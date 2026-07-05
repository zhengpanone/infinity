# Infinity Web

Infinity 的 Web/HTTP 构建块。

当前聚焦于把工作区统一错误 [`infinity_error::InfinityError`] 映射为面向 API 客户端的
HTTP 错误响应。核心类型不绑定具体 Web 框架；启用默认的 `axum` feature 后，可直接从
axum handler 返回。

## 特性

- `ApiError`：面向客户端的错误响应体，序列化为稳定的 JSON 结构
- `WebResult<T>`：Web 层统一结果类型，`?` 可自动把 `InfinityError` 转为 `ApiError`
- **服务端错误脱敏**：5xx 错误消息替换为通用文案，避免泄露内部细节；4xx 保留原始消息
- `IntoResponse` 实现（`axum` feature，默认开启），并回写 `x-request-id` 响应头

## 错误响应结构

```json
{
  "status": 404,
  "code": "not_found",
  "message": "not found: user:42",
  "request_id": "0b6f8a1e-..."
}
```

- `status`：HTTP 状态码，取自 `ErrorKind::status_code()`
- `code`：稳定错误码，取自 `ErrorKind::code()`，供客户端做程序化判断
- `message`：面向人的消息；服务端错误固定为 `internal server error`
- `request_id`：可选，无则省略

## 快速开始

```rust
use infinity_error::InfinityError;
use infinity_web::{ApiError, WebResult};

fn find_user(id: &str) -> WebResult<String> {
    if id.is_empty() {
        // InfinityError 通过 `?` / `.into()` 自动转换为 ApiError
        return Err(InfinityError::validation("id 不能为空").into());
    }
    Ok(format!("user:{id}"))
}
```

## 与 axum 集成

```rust,ignore
use axum::{routing::get, Router};
use infinity_error::InfinityError;
use infinity_web::WebResult;

async fn handler() -> WebResult<String> {
    let user = load_user().await?; // InfinityError -> ApiError -> HTTP 响应
    Ok(user)
}

let app: Router = Router::new().route("/user", get(handler));
```

## Request ID 与错误日志建议

`ApiError` 支持关联 Request ID，并在响应中回写 `x-request-id` 头，便于把一次请求的
日志与返回给客户端的错误串联起来。推荐做法：

1. 在入口中间件读取或生成 `x-request-id`（可复用 `infinity_logger::middleware::request_id`）。
2. 记录错误日志时带上 `request_id` 与稳定的 `code` 字段，方便检索与聚合：

   ```rust,ignore
   tracing::error!(
       request_id = %request_id,
       code = err.code(),
       error = %err,
       "request failed"
   );
   ```

3. 返回响应前用 `ApiError::with_request_id(request_id)` 关联同一 ID：

   ```rust,ignore
   let response = ApiError::from(&err).with_request_id(request_id);
   ```

这样客户端拿到的 `request_id` 与服务端日志一致，排查问题时可直接定位。

## 开发命令

```bash
cargo fmt -p infinity-web
cargo test -p infinity-web
cargo test -p infinity-web --no-default-features   # 核心逻辑不依赖 axum
cargo clippy -p infinity-web --all-targets -- -D warnings
```

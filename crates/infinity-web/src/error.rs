//! API 错误响应。
//!
//! [`ApiError`] 是 [`InfinityError`] 面向 HTTP 客户端的呈现形式：分类码与状态码
//! 直接取自统一错误分类，客户端错误（4xx）保留原始消息，服务端错误（5xx）出于
//! 安全考虑替换为通用消息，避免向外泄露内部实现细节。

use infinity_error::InfinityError;
use serde::Serialize;

/// 标准 Request ID 请求/响应头名称。
///
/// 与 `infinity-logger` 中间件保持一致，用于串联同一请求的日志与错误响应。
pub const REQUEST_ID_HEADER: &str = "x-request-id";

/// 服务端错误对外暴露的通用消息（避免泄露内部细节）。
const SERVER_ERROR_MESSAGE: &str = "internal server error";

/// Web 层统一 `Result`：错误已是可直接返回的 [`ApiError`]。
///
/// handler 返回该类型后，可用 `?` 让 [`InfinityError`] 自动转换为 [`ApiError`]。
pub type WebResult<T> = std::result::Result<T, ApiError>;

/// 面向 API 客户端的错误响应体。
///
/// 序列化为如下 JSON：
///
/// ```json
/// { "status": 404, "code": "not_found", "message": "not found: user:42" }
/// ```
///
/// 当携带 Request ID 时额外包含 `"request_id"` 字段。
#[derive(Debug, Clone, Serialize)]
pub struct ApiError {
    /// HTTP 状态码。
    pub status: u16,
    /// 稳定错误码（来自 [`infinity_error::ErrorKind::code`]）。
    pub code: &'static str,
    /// 面向客户端的错误消息。
    pub message: String,
    /// 关联的 Request ID，便于日志排查；为空时序列化中省略。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

impl ApiError {
    /// 从统一错误构造 API 响应（不带 Request ID）。
    ///
    /// 服务端错误（5xx）的消息会被替换为通用文案；客户端错误（4xx）保留原始消息。
    pub fn from_error(error: &InfinityError) -> Self {
        let message = if error.is_client_error() {
            error.to_string()
        } else {
            SERVER_ERROR_MESSAGE.to_owned()
        };

        Self {
            status: error.status_code(),
            code: error.code(),
            message,
            request_id: None,
        }
    }

    /// 关联一个 Request ID（链式）。
    pub fn with_request_id(mut self, request_id: impl Into<String>) -> Self {
        self.request_id = Some(request_id.into());
        self
    }
}

impl From<InfinityError> for ApiError {
    fn from(error: InfinityError) -> Self {
        Self::from_error(&error)
    }
}

impl From<&InfinityError> for ApiError {
    fn from(error: &InfinityError) -> Self {
        Self::from_error(error)
    }
}

#[cfg(feature = "axum")]
mod axum_impl {
    use super::{ApiError, REQUEST_ID_HEADER};
    use axum::{
        Json,
        http::{HeaderValue, StatusCode},
        response::{IntoResponse, Response},
    };

    impl IntoResponse for ApiError {
        fn into_response(self) -> Response {
            let status =
                StatusCode::from_u16(self.status).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
            let request_id = self.request_id.clone();

            let mut response = (status, Json(self)).into_response();

            if let Some(id) = request_id
                && let Ok(value) = HeaderValue::from_str(&id)
            {
                response.headers_mut().insert(REQUEST_ID_HEADER, value);
            }

            response
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn client_error_preserves_message() {
        let api = ApiError::from(InfinityError::not_found("user:42"));

        assert_eq!(api.status, 404);
        assert_eq!(api.code, "not_found");
        assert_eq!(api.message, "not found: user:42");
        assert!(api.request_id.is_none());
    }

    #[test]
    fn server_error_message_is_redacted() {
        let api = ApiError::from(InfinityError::internal("db password = hunter2"));

        assert_eq!(api.status, 500);
        assert_eq!(api.code, "internal");
        assert_eq!(api.message, SERVER_ERROR_MESSAGE);
        assert!(!api.message.contains("hunter2"));
    }

    #[test]
    fn with_request_id_sets_field() {
        let api = ApiError::from(InfinityError::validation("bad")).with_request_id("req-1");
        assert_eq!(api.request_id.as_deref(), Some("req-1"));
    }

    #[test]
    fn serializes_expected_shape() {
        let api = ApiError::from(InfinityError::validation("id 不能为空")).with_request_id("req-1");
        let json = serde_json::to_value(&api).unwrap();

        assert_eq!(json["status"], 400);
        assert_eq!(json["code"], "validation");
        assert_eq!(json["message"], "validation error: id 不能为空");
        assert_eq!(json["request_id"], "req-1");
    }

    #[test]
    fn request_id_omitted_when_absent() {
        let api = ApiError::from(InfinityError::not_found("x"));
        let json = serde_json::to_value(&api).unwrap();

        assert!(json.get("request_id").is_none());
    }
}

#[cfg(all(test, feature = "axum"))]
mod axum_tests {
    use super::*;
    use axum::response::IntoResponse;

    #[test]
    fn into_response_sets_status_and_request_id_header() {
        let api = ApiError::from(InfinityError::not_found("user")).with_request_id("req-9");
        let response = api.into_response();

        assert_eq!(response.status().as_u16(), 404);
        assert_eq!(response.headers().get(REQUEST_ID_HEADER).unwrap(), "req-9");
    }

    #[test]
    fn into_response_uses_500_for_server_errors() {
        let response = ApiError::from(InfinityError::database("down")).into_response();
        assert_eq!(response.status().as_u16(), 500);
    }
}

//! Infinity Web/HTTP 构建块。
//!
//! 该 crate 负责把工作区统一错误 [`infinity_error::InfinityError`] 映射为面向
//! API 客户端的 HTTP 响应。核心类型 [`ApiError`] 与 [`WebResult`] 直接实现 axum 的
//! `IntoResponse`，可从 handler 直接返回。
//!
//! ```
//! use infinity_error::InfinityError;
//! use infinity_web::{ApiError, WebResult};
//!
//! fn find_user(id: &str) -> WebResult<String> {
//!     if id.is_empty() {
//!         // InfinityError 通过 `?` 自动转换为 ApiError
//!         return Err(InfinityError::validation("id 不能为空").into());
//!     }
//!     Ok(format!("user:{id}"))
//! }
//!
//! let err = find_user("").unwrap_err();
//! assert_eq!(err.status, 400);
//! assert_eq!(err.code, "validation");
//! ```

mod api_response;
mod error;
mod pagination;

pub use api_response::ApiResponse;
pub use error::{ApiError, REQUEST_ID_HEADER, ValidationErrorDetail, WebResult};
pub use pagination::{
    PaginatedData, Pagination, PaginationInfo, PaginationLinks, PaginationParams, SortOrder,
};

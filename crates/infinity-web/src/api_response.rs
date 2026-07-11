use crate::pagination::{PaginatedData, PaginationInfo, PaginationLinks};
use crate::{ApiError, ValidationErrorDetail};
use axum::http::HeaderValue;
use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use chrono::{DateTime, Utc};
use infinity_error::InfinityError;

use serde_with::skip_serializing_none;
use std::collections::HashMap;
use std::fmt;
use std::fmt::Formatter;
use validator::ValidationErrors;

use serde::Serialize;
use utoipa::ToSchema;

/// API 响应结构
#[skip_serializing_none]
#[derive(Debug, Serialize, ToSchema)]
pub struct ApiResponse<T = ()> {
    /// 是否成功
    pub success: bool,
    /// 响应码
    pub code: String,
    /// 响应消息
    pub message: String,
    /// 响应数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
    /// 错误详情
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ApiError>,
    /// 分页信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pagination: Option<PaginationInfo>,
    /// 时间戳
    pub timestamp: DateTime<Utc>,
    /// 跟踪ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trace_id: Option<String>,
    /// 请求ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// API版本
    pub version: String,
    /// 元数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<serde_json::Value>,
}

impl<T> ApiResponse<T> {
    /// 创建成功响应
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            code: "200".to_string(),
            message: "操作成功".to_string(),
            data: Some(data),
            error: None,
            pagination: None,
            timestamp: Utc::now(),
            request_id: None,
            trace_id: None,
            version: env!("CARGO_PKG_VERSION").to_string(),
            meta: None,
        }
    }

    /// 创建成功响应（无数据）
    pub fn success_empty<S: Into<String>>(message: S) -> Self
    where
        T: Default,
    {
        Self {
            success: true,
            code: "204".to_string(),
            message: message.into(),
            data: None,
            error: None,
            pagination: None,
            timestamp: Utc::now(),
            request_id: None,
            trace_id: None,
            version: env!("CARGO_PKG_VERSION").to_string(),
            meta: None,
        }
    }

    /// 创建成功响应（自定义消息）
    pub fn success_with_message(data: T, message: impl Into<String>) -> Self {
        Self {
            success: true,
            code: "200".to_string(),
            message: message.into(),
            data: Some(data),
            error: None,
            pagination: None,
            timestamp: Utc::now(),
            request_id: None,
            trace_id: None,
            version: env!("CARGO_PKG_VERSION").to_string(),
            meta: None,
        }
    }

    pub fn created(data: T) -> Self {
        Self {
            success: true,
            code: "201".to_string(),
            message: "created".to_string(),
            data: Some(data),
            error: None,
            pagination: None,
            timestamp: Utc::now(),
            request_id: None,
            trace_id: None,
            version: env!("CARGO_PKG_VERSION").to_string(),
            meta: None,
        }
    }
    /// 创建分页响应
    pub fn paginated(data: PaginatedData<T>) -> Self {
        let pagination = data.pagination_info();

        Self {
            success: true,
            code: "200".to_string(),
            message: "查询成功".to_string(),
            data: Some(data.items),
            error: None,
            pagination: Some(pagination),
            timestamp: Utc::now(),
            request_id: None,
            trace_id: None,
            version: env!("CARGO_PKG_VERSION").to_string(),
            meta: None,
        }
    }

    /// 创建分页响应（自定义消息）
    pub fn paginated_with_message(data: PaginatedData<T>, message: impl Into<String>) -> Self {
        let mut response = Self::paginated(data);
        response.message = message.into();
        response
    }

    /// 创建错误响应
    pub fn error(status: StatusCode, error: ApiError) -> Self
    where
        T: Default,
    {
        let code = status.as_u16().to_string();
        Self {
            success: false,
            code,
            message: error.message.clone(),
            data: None,
            error: Some(error),
            pagination: None,
            timestamp: Utc::now(),
            request_id: None,
            trace_id: None,
            version: env!("CARGO_PKG_VERSION").to_string(),
            meta: None,
        }
    }
    /// 从统一错误 [`InfinityError`] 创建错误响应
    pub fn from_app_error(error: InfinityError) -> Self
    where
        T: Default,
    {
        let status_code = error.status_code();
        let http_status =
            StatusCode::from_u16(status_code).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
        let api_error = ApiError {
            status: status_code,
            code: error.code(),
            message: error.to_string(),
            details: Some(error.to_string()),
            request_id: None,
            timestamp: Utc::now(),
            validation_errors: None,
            stack_trace: None,
            documentation_url: None,
            suggestion: None,
            original_error: None,
        };
        Self::error(http_status, api_error)
    }
    /// 从验证错误创建响应
    pub fn from_validation_errors(errors: ValidationErrors) -> Self
    where
        T: Default,
    {
        let validation_errors = convert_validation_errors_to_details(&errors);

        let api_error = ApiError {
            status: StatusCode::BAD_REQUEST.as_u16(),
            // 与 InfinityError::Validation 走同一套错误码（"validation"）。
            code: infinity_error::ErrorKind::Validation.code(),
            message: "请求参数验证失败".to_string(),
            details: Some("请检查输入参数".to_string()),
            request_id: None,
            timestamp: Utc::now(),
            validation_errors: Some(validation_errors),
            stack_trace: None,
            documentation_url: None,
            suggestion: None,
            original_error: None,
        };
        Self::error(StatusCode::BAD_REQUEST, api_error)
    }
    /// 设置请求ID
    pub fn with_request_id(mut self, request_id: impl Into<String>) -> Self {
        self.request_id = Some(request_id.into());
        self
    }
    /// 设置元数据
    pub fn with_meta(mut self, meta: serde_json::Value) -> Self {
        self.meta = Some(meta);
        self
    }
    /// 设置分页链接
    pub fn with_pagination_links(mut self, links: PaginationLinks) -> Self {
        // 检查 self.pagination 是否为Some
        // 如果是Some，则将内部的值解包并赋值给 pagination 变量
        // 如果是 None，则跳过代码块（不执行任何操作）
        //
        // 为什么使用 &mut？
        // &mut self.pagination 获取 pagination 字段的可变引用
        // 这样可以在不拥有所有权的情况下修改 pagination 内部的 links 字段
        // 避免克隆整个 pagination 结构体
        if let Some(pagination) = &mut self.pagination {
            pagination.links = Some(links);
        }
        self
    }

    /// 检查是否成功
    pub fn is_success(&self) -> bool {
        self.success
    }

    /// 获取数据（如果有）
    pub fn data(&self) -> Option<&T> {
        self.data.as_ref()
    }

    /// 获取错误（如果有）
    pub fn get_error(&self) -> Option<&ApiError> {
        self.error.as_ref()
    }
}

// 为 ApiResponse 实现 From<infinity_error::Result<T>>
impl<T> From<infinity_error::Result<T>> for ApiResponse<T>
where
    T: Serialize + Default + Send + Sync + 'static,
{
    fn from(result: infinity_error::Result<T>) -> Self {
        match result {
            Ok(data) => ApiResponse::success(data),
            Err(err) => ApiResponse::from_app_error(err),
        }
    }
}

// 然后为 ApiResponse 实现 IntoResponse
impl<T> IntoResponse for ApiResponse<T>
where
    T: Serialize + Send + Sync + 'static,
{
    fn into_response(self) -> Response {
        let status = if self.success {
            // 根据 code 确定状态码
            match self.code.as_str() {
                "200" => StatusCode::OK,
                "201" => StatusCode::CREATED,
                "204" => StatusCode::NO_CONTENT,
                _ => StatusCode::OK, // 默认
            }
        } else {
            // 优先采用 ApiError 权威的 status 字段；仅在缺省（0）时才回退到按 code 推断，
            // 避免与 ErrorKind 的错误码约定重复维护而产生分歧。
            if let Some(ref error) = self.error {
                if error.status != 0 {
                    StatusCode::from_u16(error.status).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR)
                } else if let Ok(status_code) = error.code.parse::<u16>() {
                    StatusCode::from_u16(status_code).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR)
                } else {
                    StatusCode::INTERNAL_SERVER_ERROR
                }
            } else {
                StatusCode::INTERNAL_SERVER_ERROR
            }
        };
        // 构建响应头
        let mut headers = HashMap::new();
        headers.insert("Content-Type", HeaderValue::from_static("application/json"));
        headers.insert(
            "X-API-Version",
            HeaderValue::from_static(env!("CARGO_PKG_VERSION")),
        );
        headers.insert(
            "X-Request-ID",
            HeaderValue::from_str(self.request_id.as_deref().unwrap_or("unknown")).unwrap(),
        );

        if let Some(pagination) = &self.pagination {
            // 添加分页相关的头部
            headers.insert("X-Page", HeaderValue::from(pagination.page));
            headers.insert("X-Page-Size", HeaderValue::from(pagination.page_size));
            headers.insert("X-Total", HeaderValue::from(pagination.total));
            headers.insert("X-Total-Pages", HeaderValue::from(pagination.total_pages));

            // 添加分页链接头部
            if let Some(links) = &pagination.links {
                if let Some(first) = &links.first {
                    headers.insert(
                        "Link",
                        HeaderValue::from_str(&format!(r#"<{}>; rel="first""#, first)).unwrap(),
                    );
                }

                if let Some(prev) = &links.prev {
                    headers.insert(
                        "Link",
                        HeaderValue::from_str(&format!(r#"<{}>; rel="prev""#, prev)).unwrap(),
                    );
                }

                if let Some(next) = &links.next {
                    headers.insert(
                        "Link",
                        HeaderValue::from_str(&format!(r#"<{}>; rel="next""#, next)).unwrap(),
                    );
                }

                if let Some(last) = &links.last {
                    headers.insert(
                        "Link",
                        HeaderValue::from_str(&format!(r#"<{}>; rel="last""#, last)).unwrap(),
                    );
                }
            }
        }

        // 如果是 204 No Content，不应该有响应体
        if status == StatusCode::NO_CONTENT {
            status.into_response()
        } else {
            (status, Json(self)).into_response()
        }
    }
}

impl<T> fmt::Display for ApiResponse<T>
where
    T: Serialize,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if self.success {
            write!(f, "ApiResponse(success: true, message: {})", self.message)
        } else {
            write!(f, "ApiResponse(success: false, error: {:?})", self.error)
        }
    }
}

/// 把 `validator` 的字段级错误展开为 [`ValidationErrorDetail`] 列表。
pub(crate) fn convert_validation_errors_to_details(
    errors: &ValidationErrors,
) -> Vec<ValidationErrorDetail> {
    errors
        .field_errors()
        .into_iter()
        .flat_map(|(field, field_errors)| {
            field_errors.iter().map(move |err| ValidationErrorDetail {
                field: field.to_string(),
                message: err
                    .message
                    .as_ref()
                    .map(|m| m.to_string())
                    .unwrap_or_else(|| err.code.to_string()),
                code: err.code.to_string(),
                params: None,
            })
        })
        .collect()
}

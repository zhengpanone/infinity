use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::json;

#[allow(dead_code)]
pub enum ApiError {
    NotFound,
    BadRequest(String),
}
impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, msg) = match self {
            ApiError::NotFound => (StatusCode::NOT_FOUND, "not found".to_string()),
            ApiError::BadRequest(reason) => (StatusCode::BAD_REQUEST, reason),
        };
        let body = Json(json!({ "error": msg }));
        (status, body).into_response()
    }
}

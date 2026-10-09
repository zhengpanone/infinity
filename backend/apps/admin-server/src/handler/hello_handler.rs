use axum::{
    extract::Path,
    response::{Html, IntoResponse},
};

use crate::errors::ApiError;

pub async fn index() -> impl IntoResponse {
    Html(format!("Hello Admin Server"))
}

pub async fn hello() -> &'static str {
    "admin server"
}

pub async fn get_todo(Path(id): Path<u32>) -> Result<String, ApiError> {
    if id == 0 {
        return Err(ApiError::BadRequest("id 必须为正".into()));
    }
    if id > 100 {
        return Err(ApiError::NotFound);
    }
    Ok(format!("todo {id}"))
}

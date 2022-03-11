use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

// anyhow errors will easily capture errors the application produces, but the error returned
// from handlers need to implement IntoResponse, so anyhow errors need to be wrapped
pub struct MyError(pub anyhow::Error);

impl IntoResponse for MyError {
    fn into_response(self) -> Response {
        let body = Json(json!({
            "error": format!("Something went wrong: {}", self.0),
        }));

        (StatusCode::INTERNAL_SERVER_ERROR, body).into_response()
    }
}

impl From<anyhow::Error> for MyError {
    fn from(error: anyhow::Error) -> Self {
        MyError(error.into())
    }
}

impl From<sqlx::Error> for MyError {
    fn from(error: sqlx::Error) -> Self {
        MyError(error.into())
    }
}

impl From<reqwest::Error> for MyError {
    fn from(error: reqwest::Error) -> Self {
        MyError(error.into())
    }
}

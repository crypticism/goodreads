use axum::response::{IntoResponse, Response};

use crate::templates::{ErrorTemplate, HtmlTemplate};

// anyhow errors will easily capture errors the application produces, but the error returned
// from handlers need to implement IntoResponse, so anyhow errors need to be wrapped
pub struct MyError(pub anyhow::Error);

impl IntoResponse for MyError {
    fn into_response(self) -> Response {
        HtmlTemplate(ErrorTemplate {
            message: self.0.to_string(),
        })
        .into_response()
    }
}

impl From<anyhow::Error> for MyError {
    fn from(error: anyhow::Error) -> Self {
        MyError(error)
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

pub fn make_err(e: impl std::error::Error, msg: &str) -> anyhow::Error {
    anyhow::anyhow!(format!("{}: {}", msg, e))
}

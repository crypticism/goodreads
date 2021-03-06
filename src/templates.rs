use askama::Template;
use axum::response::{Html, IntoResponse, Response};
use reqwest::StatusCode;

#[derive(Template)]
#[template(path = "subscribe.html")]
pub struct SubscribeTemplate {
    pub id: String,
    pub profile_id: String,
    pub update_picture: bool,
    pub update_status: bool,
    pub update_title: bool,
}

#[derive(Template)]
#[template(path = "success.html")]
pub struct SuccessTemplate {}

#[derive(Template)]
#[template(path = "refresh.html")]
pub struct RefreshTemplate {}

#[derive(Template)]
#[template(path = "error.html")]
pub struct ErrorTemplate {
    pub message: String,
}

pub struct HtmlTemplate<T>(pub T);

impl<T> IntoResponse for HtmlTemplate<T>
where
    T: Template,
{
    fn into_response(self) -> Response {
        match self.0.render() {
            Ok(html) => Html(html).into_response(),
            Err(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to render template. Error: {}", err),
            )
                .into_response(),
        }
    }
}

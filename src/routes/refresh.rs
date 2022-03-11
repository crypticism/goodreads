use axum::{extract::Extension, response::IntoResponse};

use sqlx::{Pool, Postgres};

use super::shared::update_profile;
use crate::error::MyError;
use crate::structs::AppUser;
use crate::templates::{HtmlTemplate, RefreshTemplate};

#[axum_macros::debug_handler]
pub async fn refresh(
    Extension(pool): Extension<Pool<Postgres>>,
) -> Result<impl IntoResponse, MyError> {
    let users = sqlx::query_as!(AppUser, "SELECT * FROM users")
        .fetch_all(&pool)
        .await?;

    for user in users {
        update_profile(user, &pool).await?;
    }

    Ok(HtmlTemplate(RefreshTemplate {}))
}

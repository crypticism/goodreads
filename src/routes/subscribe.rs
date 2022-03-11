use std::{collections::HashMap, sync::Arc};

use axum::{
    extract::{Extension, Query},
    response::IntoResponse,
};
use reqwest;

use sqlx::{Pool, Postgres};

use crate::error::MyError;
use crate::structs::{Authorization, Context};
use crate::templates::{HtmlTemplate, SubscribeTemplate};

#[axum_macros::debug_handler]
pub async fn subscribe(
    Query(params): Query<HashMap<String, String>>,
    Extension(context): Extension<Arc<Context>>,
    Extension(pool): Extension<Pool<Postgres>>,
) -> Result<impl IntoResponse, MyError> {
    // Uses authorization code to get the access token that will allow modifying a user's profile
    let url = format!(
        "https://slack.com/api/oauth.v2.access?code={}&client_id={}&client_secret={}",
        params
            .get("code")
            .ok_or(anyhow::anyhow!("Missing code query param"))?,
        context.client_id,
        context.client_secret
    );
    let response = reqwest::get(url).await?.json::<Authorization>().await?;

    // Insert user into the database if it doesn't exist
    // If the user does exist, update the access token in case it has changed
    let user = sqlx::query!(
        r#"
            INSERT INTO users
            (id, scope, access_token, token_type, update_picture, update_status, update_title)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            ON CONFLICT (id)
            DO UPDATE
            SET access_token = $8
            RETURNING *
        "#,
        &response.authed_user.id,
        &response.authed_user.scope,
        &response.authed_user.access_token,
        &response.authed_user.token_type,
        false,
        false,
        false,
        &response.authed_user.access_token,
    )
    .fetch_one(&pool)
    .await?;

    let profile_id = match user.profile_id {
        Some(val) => val,
        None => String::new(),
    };

    Ok(HtmlTemplate(SubscribeTemplate {
        id: user.id,
        profile_id,
        update_picture: user.update_picture,
        update_status: user.update_status,
        update_title: user.update_title,
    }))
}

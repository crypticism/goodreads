use axum::{
    extract::{Extension, Form},
    response::IntoResponse,
};
use sqlx::{Pool, Postgres};

use super::shared::{checkbox_to_bool, update_profile};
use crate::error::{make_err, MyError};
use crate::structs::{AppUser, UpdateProfileId};
use crate::templates::{HtmlTemplate, SuccessTemplate};

#[axum_macros::debug_handler]
pub async fn update_settings(
    Extension(pool): Extension<Pool<Postgres>>,
    form: Form<UpdateProfileId>,
) -> Result<impl IntoResponse, MyError> {
    let update_picture = checkbox_to_bool(&form.update_picture);
    let update_status = checkbox_to_bool(&form.update_status);
    let update_title = checkbox_to_bool(&form.update_title);

    let user = sqlx::query_as!(
        AppUser,
        r#"
            UPDATE users
            SET (profile_id, update_picture, update_status, update_title) = ($1, $2, $3, $4)
            WHERE id = $5
            RETURNING *
        "#,
        form.profile_id,
        update_picture,
        update_status,
        update_title,
        form.id
    )
    .fetch_one(&pool)
    .await
    .map_err(|e| make_err(e, "Unable to update user"))?;

    update_profile(user, &pool).await?;

    Ok(HtmlTemplate(SuccessTemplate {}))
}

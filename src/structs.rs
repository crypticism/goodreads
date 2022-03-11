use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Clone, Debug)]
#[allow(dead_code)]
pub struct Context {
    pub client_id: String,
    pub client_secret: String,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct Authorization {
    ok: bool,
    app_id: String,
    pub authed_user: User,
    team: Team,
    enterprise: Option<bool>,
    is_enterprise_install: bool,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct User {
    pub id: String,
    pub scope: String,
    pub access_token: String,
    pub token_type: String,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct Team {
    id: String,
    name: String,
}

#[derive(Debug, Deserialize)]
pub struct ResponseSuccess {
    pub ok: bool,
    #[serde(default)]
    pub error: String,
}

#[derive(Debug)]
pub struct AppConfig {
    pub client_id: String,
    pub client_secret: String,
    pub port: u16,
    pub database_url: String,
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct AppUser {
    pub id: String,
    pub scope: String,
    pub access_token: String,
    pub token_type: String,
    pub profile_id: Option<String>,
    pub title: Option<String>,
    pub update_picture: bool,
    pub update_status: bool,
    pub update_title: bool,
}

#[derive(Debug, Deserialize)]
pub struct UpdateProfileId {
    pub id: String,
    pub profile_id: String,
    pub update_picture: Checkbox,
    pub update_status: Checkbox,
    pub update_title: Checkbox,
}

pub type Checkbox = Option<String>;

#[derive(Debug, Deserialize)]
pub struct UserProfileGet {
    pub profile: Profile,
}

#[derive(Debug, Deserialize)]
pub struct Profile {
    pub status_emoji: String,
}

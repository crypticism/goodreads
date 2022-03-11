mod db;
mod error;
mod routes;
mod structs;
mod templates;

use std::{env, net::SocketAddr, sync::Arc};

use axum::{
    extract::Extension,
    routing::{get, post},
    Router,
};
use axum_server::tls_rustls::RustlsConfig;
use dotenv::dotenv;

use db::init_db;
use routes::{refresh::refresh, subscribe::subscribe, update::update_settings};
use structs::*;

#[tokio::main]
async fn main() {
    let config = init();

    let addr = SocketAddr::from(([127, 0, 0, 1], config.port));

    let context = Arc::new(Context {
        client_id: config.client_id.clone(),
        client_secret: config.client_secret.clone(),
    });

    let pool = init_db(&config).await;

    let rustls_config = RustlsConfig::from_pem_file("conf/cert.pem", "conf/key.pem")
        .await
        .unwrap();

    let app = Router::new()
        .route("/", get(subscribe))
        .route("/refresh", get(refresh))
        .route("/update_settings", post(update_settings))
        .layer(Extension(pool))
        .layer(Extension(context));

    axum_server::bind_rustls(addr, rustls_config)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

fn init() -> AppConfig {
    dotenv().ok();

    let client_id = env::var("CLIENT_ID").expect("CLIENT_ID must be set");
    let client_secret = env::var("CLIENT_SECRET").expect("CLIENT_SECRET must be set");
    let port = env::var("PORT")
        .unwrap_or_else(|_| String::from("3000"))
        .parse::<u16>()
        .unwrap();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    AppConfig {
        client_id,
        client_secret,
        port,
        database_url,
    }
}

use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::{routing::get, Router};
use sea_orm::{DbConn, DbErr};
use serde::{Deserialize, Serialize};
use tokio::fs;
use std::sync::Arc;
use uuid::Uuid;

use super::settings::Settings;

use self::auth::AuthState;
use self::notifier::Notifier;
use self::summarizer::ya_gpt::YaGPT;

mod auth;
mod db;
mod messages;
mod notifier;
mod streams;
mod summarizer;
mod tasks;
mod users;

pub async fn run(settings: &Settings) {
    let state = AppState::new(settings).await;

    // TODO: Deal with it later
    // Migrator::up(state.db.as_ref(), None).await.unwrap();
    // info!("Migrator finished");

    // Start tasks processor
    // tasks::executor::run(&state);

    let app = Router::new()
        .nest(
            "/api",
            Router::new()
                .route("/healthz", get(|| async {}))
                .nest("/auth", auth::router())
                .nest("/messages", messages::router())
                .nest("/streams", streams::router())
                .nest("/users", users::router()),
        )
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(&settings.http.endpoint)
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[derive(Debug)]
pub enum AppError {
    EntityNotFound,
    Forbidden,
    BadRequest,
    Database,
    Json,
    Verify,
}

impl From<DbErr> for AppError {
    fn from(_: DbErr) -> Self {
        AppError::Database
    }
}

impl From<serde_json::Error> for AppError {
    fn from(_: serde_json::Error) -> Self {
        AppError::Json
    }
}

impl From<ecdsa::Error> for AppError {
    fn from(_: ecdsa::Error) -> Self {
        AppError::Verify
    }
}

impl From<p256::pkcs8::spki::Error> for AppError {
    fn from(_: p256::pkcs8::spki::Error) -> Self {
        AppError::Verify
    }
}

impl From<jsonwebtoken::errors::Error> for AppError {
    fn from(_: jsonwebtoken::errors::Error) -> Self {
        AppError::Json
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = match self {
            AppError::EntityNotFound => StatusCode::NOT_FOUND,
            AppError::Forbidden => StatusCode::FORBIDDEN,
            _ => StatusCode::BAD_REQUEST,
        };

        (status).into_response()
    }
}

#[derive(Clone)]
pub struct AppState {
    auth_state: AuthState,
    pub db: Arc<DbConn>,
    pub notifier: Arc<Notifier>,
    // pub auth_public_key: Arc<Vec<u8>>,
    pub ya_gpt: Arc<YaGPT>,
}

impl AppState {
    async fn new(settings: &Settings) -> Self {
        let db = Arc::new(db::create_pool(&settings.database).await);

        // let rp_origin = Url::parse(&env::var("AUTH_RP_ORIGIN").unwrap()).unwrap();
        // let rp_id = env::var("AUTH_RP_ID").unwrap();
        // let builder = WebauthnBuilder::new(&rp_id, &rp_origin)
        //     .unwrap()
        //     .rp_name("Flux");

        // let webauthn = Arc::new(builder.build().unwrap());
        let notifier = Arc::new(Notifier::new(&settings.notifier.endpoint, db.clone()).await);

        // let auth_public_key = env::var("AUTH_PUBLIC_KEY_FILE").unwrap();
        // let auth_public_key = Arc::new(
        //     fs::read_to_string(auth_public_key)
        //         .await
        //         .unwrap()
        //         .into_bytes(),
        // );

        // let auth_public_key = Arc::new(vec![]);
        // let settings = Arc::new(settings);

        // dbg!(&settings);

        let auth_state = AuthState {
            rp_id: settings.auth.rp_id.clone(),
            rp_name: settings.auth.rp_name.clone(),
            public_key: fs::read_to_string(&settings.auth.public_key_file).await.unwrap_or_default().into_bytes(),
            private_key: fs::read_to_string(&settings.auth.private_key_file).await.unwrap_or_default().into_bytes(),
        };

        let ya_gpt = Arc::new(YaGPT::new(&settings.ya_gpt));

        AppState {
            auth_state,
            db,
            notifier,
            ya_gpt,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AppSession {
    pub user: Option<users::entities::user::Model>,
}

#[derive(Debug, Deserialize, Serialize)]
struct AppToken {
    pub sub: Uuid,
    pub exp: i64,
}

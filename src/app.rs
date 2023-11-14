use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::{routing::get, Router};
use migration::{Migrator, MigratorTrait};
use sea_orm::DatabaseConnection;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::{env, net::SocketAddr, str::FromStr};
use tracing::info;
use uuid::Uuid;
use webauthn_rs::prelude::Url;
use webauthn_rs::{Webauthn, WebauthnBuilder};

use self::notifier::Notifier;

mod db;

mod auth;
mod messages;
// mod session;
mod notifier;
mod streams;
mod summarizer;
mod tasks;
mod users;

pub async fn run() {
    let state = AppState::new().await;

    // TODO: Deal with it later
    Migrator::up(state.db.as_ref(), None).await.unwrap();
    info!("Migrator finished");

    // Start tasks processor
    tasks::executor::run(&state).await;

    let app = Router::new()
        .route("/healthz", get(|| async {}))
        .nest("/auth", auth::router())
        // .nest("/session", session::router())
        .nest("/messages", messages::router())
        .nest("/streams", streams::router())
        .nest("/users", users::router())
        .with_state(state);

    let addr = SocketAddr::from_str(&env::var("APP_ADDR").unwrap()).unwrap();

    info!("App start on {}", &addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

pub enum AppError {
    EntityNotFound,
    Forbidden,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = match self {
            AppError::EntityNotFound => StatusCode::NOT_FOUND,
            AppError::Forbidden => StatusCode::FORBIDDEN,
        };

        (status).into_response()
    }
}

#[derive(Clone)]
pub struct AppState {
    pub db: Arc<DatabaseConnection>,
    pub webauthn: Arc<Webauthn>,
    pub notifier: Arc<Notifier>,
}

impl AppState {
    async fn new() -> Self {
        let db = Arc::new(db::create_pool(&env::var("DATABASE_URL").unwrap()).await);

        let rp_origin = Url::parse(&env::var("AUTH_RP_ORIGIN").unwrap()).unwrap();
        let rp_id = env::var("AUTH_RP_ID").unwrap();
        let builder = WebauthnBuilder::new(&rp_id, &rp_origin)
            .unwrap()
            .rp_name("Flux");

        let webauthn = Arc::new(builder.build().unwrap());
        let notifier = Arc::new(Notifier::new(env::var("NATS_ADDR").unwrap()).await);

        AppState {
            db,
            webauthn,
            notifier,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AppSession {
    pub user: Option<users::entities::user::Model>,
}

#[derive(Debug, Deserialize)]
struct JwtUser {
    pub sub: Uuid,
}

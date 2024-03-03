use axum::Router;

use super::AppState;

pub mod entities;
pub mod push_subscriptions;
pub mod repo;

pub fn router() -> Router<AppState> {
    Router::new().nest("/push-subscriptions", push_subscriptions::router())
}

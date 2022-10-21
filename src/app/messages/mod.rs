use axum::{
    routing::{get, post},
    Json, Router,
};

use self::data::IndexData;

use super::User;

mod data;
mod service;

#[cfg(test)]


pub fn router() -> Router {
    Router::new()
        .route("/", get(index))
        .route("/", post(create))
}

async fn index() -> Json<IndexData> {
    Json(service::index())
}

async fn create(user: User) -> Json<IndexData> {
    Json(service::create(user))
}

#[cfg(test)]
mod tests {
    use crate::app::sessions::service::create_token;

    use super::*;
    use axum::{
        body::Body,
        http::{self, Request, StatusCode},
    };
    use dotenv::dotenv;
    use sea_orm::prelude::Uuid;
    use tower::ServiceExt;

    #[tokio::test]
    async fn test_index_get() {
        let app = router();
        let response = app
            .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_create_post() {
        dotenv().ok();

        let app = router();
        let response = app
            .oneshot(
                Request::builder()
                    .method(http::Method::POST)
                    .header(
                        http::header::AUTHORIZATION,
                        ["Bearer", &create_token(Uuid::new_v4())].join(" "),
                    )
                    .uri("/")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }
}

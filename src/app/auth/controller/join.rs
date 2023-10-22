use axum::{extract::State, Json};
use sea_orm::{ActiveValue::NotSet, Set};
use uuid::Uuid;
use validator::Validate;
use webauthn_rs::prelude::{CreationChallengeResponse, Passkey, RequestChallengeResponse};

use crate::app::{auth::data::join::RequestData, users::repo::UsersRepo, AppState};

use super::{
    super::{data::join::ResponseData, entities, repo::AuthRepo},
    AuthController,
};

impl AuthController {
    pub async fn join(
        State(state): State<AppState>,
        Json(request_data): Json<RequestData>,
    ) -> Json<ResponseData> {
        request_data.validate().unwrap();
        let email = request_data.email.to_lowercase();

        let user = UsersRepo::find_user_by_email(state.db.as_ref(), &email).await;

        match user {
            Some(user) => Json(
                AuthController::create_passkey_auth(state, user)
                    .await
                    .into(),
            ),
            None => Json(
                AuthController::create_passkey_reg(state, &email)
                    .await
                    .into(),
            ),
        }
    }

    async fn create_passkey_auth(
        state: AppState,
        user: entities::user::Model,
    ) -> (RequestChallengeResponse, entities::auth_state::Model) {
        let passkeys: Vec<Passkey> = serde_json::from_value(user.passkeys).unwrap();
        let (challenge, passkey) = state
            .webauthn
            .start_passkey_authentication(&passkeys)
            .unwrap();

        let auth_state = AuthRepo::create_auth_state(
            state.db.as_ref(),
            entities::auth_state::ActiveModel {
                id: NotSet,
                user_id: Set(user.id),
                email: Set(user.email),
                state: Set(serde_json::to_value(passkey).unwrap()),
                created_at: NotSet,
            },
        )
        .await;

        return (challenge, auth_state);
    }

    async fn create_passkey_reg(
        state: AppState,
        email: &String,
    ) -> (CreationChallengeResponse, entities::auth_state::Model) {
        let user_id = Uuid::now_v7();
        let (challenge, passkey) = state
            .webauthn
            .start_passkey_registration(user_id, "UNAME_1", "UNAME_2", None)
            .unwrap();

        let auth_state = AuthRepo::create_auth_state(
            state.db.as_ref(),
            entities::auth_state::ActiveModel {
                id: NotSet,
                user_id: Set(user_id),
                email: Set(email.to_string()),
                state: Set(serde_json::to_value(passkey).unwrap()),
                created_at: NotSet,
            },
        )
        .await;

        return (challenge, auth_state);
    }
}

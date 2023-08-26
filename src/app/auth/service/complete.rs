use sea_orm::{DatabaseConnection, ModelTrait, TransactionTrait};
use webauthn_rs::{
    prelude::{PasskeyRegistration, RegisterPublicKeyCredential},
    Webauthn,
};

use crate::app::{auth::repo::AuthRepo, users::repo::UsersRepo};

use super::{super::entities, AuthService};

impl AuthService {
    pub async fn complete(
        db: &DatabaseConnection,
        webauthn: &Webauthn,
        auth_state_id: String,
        reg: RegisterPublicKeyCredential,
    ) -> entities::user::Model {
        let txn = db.begin().await.unwrap();
        let auth_state = AuthRepo::find_auth_state_by_id(&txn, &auth_state_id).await;

        let passkey_registration: PasskeyRegistration =
            serde_json::from_value(auth_state.state.clone()).unwrap();

        let passkey = webauthn
            .finish_passkey_registration(&reg, &passkey_registration)
            .unwrap();

        let user = UsersRepo::create_user(
            &txn,
            auth_state.user_id.clone(),
            auth_state.email.clone(),
            Some(passkey),
        )
        .await;

        auth_state.delete(&txn).await.unwrap();

        txn.commit().await.unwrap();

        user
    }
}

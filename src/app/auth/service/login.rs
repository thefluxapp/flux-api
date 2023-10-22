use sea_orm::{DatabaseConnection, ModelTrait, TransactionTrait};
use webauthn_rs::{
    prelude::{Passkey, PasskeyAuthentication, PublicKeyCredential},
    Webauthn,
};

use crate::app::{auth::repo::AuthRepo, users::repo::UsersRepo};

use super::{super::entities, AuthService};

impl AuthService {
    pub async fn login(
        db: &DatabaseConnection,
        webauthn: &Webauthn,
        auth_state_id: &String,
        auth: &PublicKeyCredential,
    ) -> entities::user::Model {
        let txn = db.begin().await.unwrap();
        let auth_state = AuthRepo::find_auth_state_by_id(&txn, auth_state_id).await;

        let passkey_authentication: PasskeyAuthentication =
            serde_json::from_value(auth_state.state.clone()).unwrap();

        let auth_result = webauthn
            .finish_passkey_authentication(auth, &passkey_authentication)
            .unwrap();

        let user = UsersRepo::find_user_by_id(&txn, auth_state.user_id)
            .await
            .unwrap();

        let mut passkeys: Vec<Passkey> = serde_json::from_value(user.passkeys.clone()).unwrap();
        for passkey in passkeys.iter_mut() {
            passkey.update_credential(&auth_result);
        }

        UsersRepo::update_user_passkeys(&txn, user.clone().into(), passkeys).await;

        auth_state.delete(&txn).await.unwrap();

        txn.commit().await.unwrap();

        user
    }
}

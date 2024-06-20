use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
use chrono::{Days, Utc};
use ecdsa::der::Signature;
use ecdsa::signature::Verifier;
use ecdsa::{elliptic_curve::pkcs8::DecodePublicKey as _, VerifyingKey};
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use rand::RngCore as _;
use sea_orm::{ActiveValue::NotSet, DbConn, Set, TransactionTrait};
use sha2::{Digest as _, Sha256};
use url::Url;
use uuid::Uuid;

use crate::app::{AppError, AppToken};

use super::{
    data::{
        ClientData, CompleteRequestData, CredentialCreationOptions, CredentialRequestOptions,
        JoinResponseData, LoginRequestData, PublicKeyCredentialCreationOptions,
        PublicKeyCredentialDescriptor, PublicKeyCredentialParameters,
        PublicKeyCredentialRequestOptions, PublicKeyCredentialRpEntity,
        PublicKeyCredentialUserEntity,
    },
    entities::{self},
    repo, AuthState,
};

pub async fn join(
    db: &DbConn,
    auth_state: &AuthState,
    email: String,
) -> Result<JoinResponseData, AppError> {
    let mut challenge = vec![0u8; 64];
    rand::thread_rng().fill_bytes(&mut challenge);

    match repo::find_user_by_email_with_credentials(db, &email).await? {
        Some((user, user_credentials)) => {
            let cro = PublicKeyCredentialRequestOptions {
                challenge,
                rp_id: Some(auth_state.rp_id.clone()),
                allow_credentials: user_credentials
                    .into_iter()
                    .map(|user_credential| PublicKeyCredentialDescriptor {
                        id: user_credential.id,
                        tp: "public-key".to_string(),
                        transports: vec!["internal".to_string()],
                    })
                    .collect(),
            };

            repo::create_user_challenge(db, {
                entities::user_challenge::ActiveModel {
                    id: Set(URL_SAFE_NO_PAD.encode(cro.challenge.clone())),
                    user_id: Set(user.id),
                    user_name: NotSet,
                    created_at: Set(Utc::now().naive_utc()),
                }
            })
            .await?;

            Ok(JoinResponseData::Request(CredentialRequestOptions {
                public_key: cro,
            }))
        }
        None => {
            let cco = PublicKeyCredentialCreationOptions {
                challenge,
                // TODO: remove hardcode
                pub_key_cred_params: vec![
                    PublicKeyCredentialParameters {
                        alg: -7,
                        tp: "public-key".to_string(),
                    },
                    PublicKeyCredentialParameters {
                        alg: -257,
                        tp: "public-key".to_string(),
                    },
                ],
                rp: PublicKeyCredentialRpEntity {
                    id: Some(auth_state.rp_id.clone()),
                    name: auth_state.rp_name.clone(),
                },
                user: PublicKeyCredentialUserEntity {
                    id: Uuid::now_v7(),
                    name: email.to_lowercase().to_string(),
                    display_name: email,
                },
            };

            repo::create_user_challenge(db, {
                entities::user_challenge::ActiveModel {
                    id: Set(URL_SAFE_NO_PAD.encode(cco.challenge.clone())),
                    user_id: Set(cco.user.id),
                    user_name: Set(Some(cco.user.name.clone())),
                    created_at: Set(Utc::now().naive_utc()),
                }
            })
            .await?;

            Ok(JoinResponseData::Creation(CredentialCreationOptions {
                public_key: cco,
            }))
        }
    }
}

pub async fn complete(
    db: &DbConn,
    auth_state: &AuthState,
    data: CompleteRequestData,
) -> Result<entities::user::Model, AppError> {
    let client_data: ClientData = data.credential.response.client_data;

    validate_origin(&client_data.origin, &auth_state.rp_id)?;
    validate_tp(&client_data.tp, "webauthn.create")?;

    let txn = db.begin().await?;

    let user_challenge = match repo::find_user_challengle(&txn, &client_data.challenge).await? {
        Some(user_challenge) => Ok(user_challenge),
        None => Err(AppError::EntityNotFound),
    }?;

    let email = match user_challenge.user_name.clone() {
        Some(email) => Ok(email),
        None => Err(AppError::EntityNotFound),
    }?;

    let user = repo::create_user(
        &txn,
        entities::user::Model {
            id: user_challenge.user_id,
            email,
            first_name: Some(data.first_name.clone()),
            last_name: Some(data.last_name.clone()),
            // passkeys: todo!(),
            created_at: chrono::Utc::now().naive_utc(),
            updated_at: chrono::Utc::now().naive_utc(),
        },
    )
    .await?;

    repo::create_user_credential(
        &txn,
        entities::user_credential::Model {
            id: data.credential.id,
            user_id: user.id,
            public_key: data.credential.response.public_key,
            public_key_algorithm: data.credential.response.public_key_algorithm,
            created_at: chrono::Utc::now().naive_utc(),
        },
    )
    .await?;

    repo::delete_user_challengle(&txn, user_challenge).await?;

    txn.commit().await?;

    Ok(user)
}

pub async fn login(
    db: &DbConn,
    auth_state: &AuthState,
    data: LoginRequestData,
) -> Result<entities::user::Model, AppError> {
    let client_data: ClientData =
        serde_json::from_slice(&data.credential.response.client_data_json)?;

    validate_origin(&client_data.origin, &auth_state.rp_id)?;
    validate_tp(&client_data.tp, "webauthn.get")?;

    let txn = db.begin().await?;

    let client_data_json_hash = Sha256::digest(&data.credential.response.client_data_json).to_vec();

    let (user_credential, user_challenge) =
        repo::find_user_credential_and_challenge(&txn, &data.credential.id, &client_data.challenge)
            .await?;

    let verifying_key: VerifyingKey<p256::NistP256> =
        ecdsa::VerifyingKey::from_public_key_der(&user_credential.public_key)?;

    let mut message: Vec<u8> = data.credential.response.authenticator_data;
    message.extend(&client_data_json_hash);

    let signature = Signature::from_bytes(&data.credential.response.signature)?;
    verifying_key.verify(&message, &signature)?;

    let user_id = user_challenge.user_id.clone();
    repo::delete_user_challengle(&txn, user_challenge).await?;

    txn.commit().await?;

    let user = repo::find_user_by_id(db, user_id).await?;

    Ok(user)
}

pub fn create_jwt(auth_state: &AuthState, user: &entities::user::Model) -> Result<String, AppError> {
    let claims = AppToken {
        sub: user.id,
        exp: chrono::Utc::now()
            .checked_add_days(Days::new(600))
            .ok_or_else(|| AppError::BadRequest)?
            .timestamp(),
    };

    let jwt = encode(
        &Header::new(Algorithm::RS256),
        &claims,
        &EncodingKey::from_rsa_pem(&auth_state.private_key).unwrap(),
    )?;
    Ok(jwt)
}

fn validate_origin(origin: &str, expected: &str) -> Result<(), AppError> {
    // TODO: rewrite it!

    return match Url::parse(&origin) {
        Ok(url) => match url.host() {
            Some(host) => {
                if host.to_string() == expected {
                    Ok(())
                } else {
                    Err(AppError::Verify)
                }
            }
            None => Err(AppError::Verify),
        },
        Err(_) => Err(AppError::Verify),
    };
}

fn validate_tp(tp: &str, expected: &str) -> Result<(), AppError> {
    if tp != expected {
        return Err(AppError::Verify);
    };

    Ok(())
}

#[cfg(test)]
mod tests {
    use base64::engine::general_purpose::URL_SAFE_NO_PAD;
    use base64::Engine;

    use super::*;

    #[test]
    fn hash_user_id_test() {
        let user_id = Uuid::parse_str("018f6830-3a33-7857-8239-b01d71b1c914").unwrap();

        assert_eq!(
            URL_SAFE_NO_PAD.encode(user_id.as_bytes()),
            "AY9oMDozeFeCObAdcbHJFA"
        );
    }

    #[test]
    fn validate_origin_test() {
        assert!(validate_origin("http://example.com/", "example.com").is_ok());
        assert!(validate_origin("http://example.com/path", "example.com").is_ok());
        assert!(validate_origin("http://example.com:3000/", "example.com").is_ok());
        assert!(validate_origin("http://localhost:3000/", "localhost").is_ok());
        assert!(validate_origin("http://sub.example.com/", "example.com").is_err());
        assert!(validate_origin("http://not-example.com/", "example.com").is_err());
    }
}

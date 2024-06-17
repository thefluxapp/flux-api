use std::{
    env,
    sync::Arc,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
use chrono::Utc;
use ecdsa::der::Signature;
use rand::RngCore as _;
// use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
// use passkey::{
//     authenticator::{Authenticator, UserValidationMethod},
//     client::Client,
//     types::{
//         ctap2::Aaguid,
//         rand::random_vec,
//         webauthn::{
//             AttestationConveyancePreference, AuthenticatorAttestationResponse,
//             CredentialCreationOptions, CredentialRequestOptions, PublicKeyCredential,
//             PublicKeyCredentialCreationOptions, PublicKeyCredentialParameters,
//             PublicKeyCredentialRequestOptions, PublicKeyCredentialRpEntity,
//             PublicKeyCredentialType, PublicKeyCredentialUserEntity, UserVerificationRequirement,
//         },
//         Passkey,
//     },
// };
use ecdsa::signature::Verifier;
use ecdsa::{elliptic_curve::pkcs8::DecodePublicKey as _, VerifyingKey};
use sea_orm::{ActiveValue::NotSet, DbConn, ModelTrait, Set, TransactionTrait};
use serde::Serialize;
use sha2::{Digest as _, Sha256};
use tokio::fs;
use url::Url;
use uuid::Uuid;

use crate::app::{users::entities::user_challenge, AppError};

use super::{
    data::{
        CompleteRequestData, CompleteResponseData, CredentialCreationOptions,
        CredentialRequestOptions, JoinRequestData, JoinResponseData, LoginRequestData,
        LoginResponseData, PublicKeyCredentialCreationOptions, PublicKeyCredentialDescriptor,
        PublicKeyCredentialParameters, PublicKeyCredentialRequestOptions,
        PublicKeyCredentialRpEntity, PublicKeyCredentialUserEntity,
    },
    entities::{self, user_credential},
    repo, AuthState,
};

// mod complete;
// mod login;

#[derive(Debug, Serialize)]
pub struct AuthPayload {
    pub sub: Uuid,
    pub exp: u128,
}

// pub struct AuthService {}

// impl AuthService {
//     pub async fn generate_token(sub: Uuid) -> String {
//         let payload = AuthPayload {
//             sub,
//             exp: (SystemTime::now() + Duration::new(60 * 60 * 24 * 365, 0))
//                 .duration_since(UNIX_EPOCH)
//                 .unwrap()
//                 .as_millis(),
//         };

//         // TODO: Do not read from file every time
//         let auth_private_key = env::var("AUTH_PRIVATE_KEY_FILE").unwrap();
//         let auth_private_key = fs::read_to_string(auth_private_key).await.unwrap();

//         encode(
//             &Header::new(Algorithm::RS256),
//             &payload,
//             &EncodingKey::from_rsa_pem(&auth_private_key.into_bytes()).unwrap(),
//         )
//         .unwrap()
//     }
// }

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
                // user_verification: "preferred".to_string(),
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
    data: CompleteRequestData,
) -> Result<CompleteResponseData, AppError> {
    if data.credential.response.client_data.tp != "webauthn.create" {
        Err(AppError::BadRequest)?;
    }

    // TODO: Check origin

    let txn = db.begin().await?;

    let user_challenge =
        match repo::find_user_challengle(&txn, &data.credential.response.client_data.challenge)
            .await?
        {
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

    Ok(CompleteResponseData {})
}

pub async fn login(db: &DbConn, data: LoginRequestData) -> Result<LoginResponseData, AppError> {
    let user_credential = repo::find_user_credential(db, &data.credential.id).await?;

    let verifying_key: VerifyingKey<p256::NistP256> =
        ecdsa::VerifyingKey::from_public_key_der(&user_credential.public_key).unwrap();

    let client_data_json_hash = Sha256::digest(&data.credential.response.client_data_json).to_vec();

    dbg!(&data.credential.response.authenticator_data);
    dbg!(&client_data_json_hash);

    let mut message: Vec<u8> = data.credential.response.authenticator_data;
    message.extend(&client_data_json_hash);

    dbg!(&message);

    let signature = Signature::from_bytes(&data.credential.response.signature).unwrap();

    let result = verifying_key.verify(&message, &signature);

    dbg!(&result);

    Ok(LoginResponseData {})
}

#[cfg(test)]
mod tests {
    use base64::engine::general_purpose::URL_SAFE_NO_PAD;
    use base64::Engine;

    use ecdsa::der::Signature;
    use ecdsa::signature::Verifier;
    use ecdsa::{elliptic_curve::pkcs8::DecodePublicKey as _, VerifyingKey};
    // use sha2::;
    use sha2::{Digest, Sha256};

    use super::*;

    #[test]
    fn hash_user_id() {
        let user_id = Uuid::parse_str("018f6830-3a33-7857-8239-b01d71b1c914").unwrap();

        assert_eq!(
            URL_SAFE_NO_PAD.encode(user_id.as_bytes()),
            "AY9oMDozeFeCObAdcbHJFA"
        );
    }

    #[test]
    fn sign_test() {
        // let verifying_key = "MFkwEwYHKoZIzj0CAQYIKoZIzj0DAQcDQgAEaBtE5hCryzaXqsR_HxOn4mJfl54fUwqjXBQV7ACDuRD3blNTjSljOh4IxHrJwzJ_hPE5j5TGfuzU5ucDbN0pfA";
        // let client_data_json = "eyJ0eXBlIjoid2ViYXV0aG4uZ2V0IiwiY2hhbGxlbmdlIjoicTRCbHBGNUN2TmlIeDVxOGtYMUpPR2hWWkROMW1pRlBNREpwTHF1VlkyWEMtSTdTelJvaHc3N3pSdk1KY1BkYWpNWlJjYkdzVEZXRnFKWnpMdkhuaHciLCJvcmlnaW4iOiJodHRwczovL3dlYmF1dGhuLmlvIiwiY3Jvc3NPcmlnaW4iOmZhbHNlfQ";
        // let authenticator_data = "dKbqkhPJnC90siSSsyDPQCYqlMGpUKA5fyklC2CEHvAFAAAAAA";
        // let signature = "MEUCIBYjb1n5XAZ_wuWLKT3nfjdOb6Ai7Hh0v2jl0DG-BFd1AiEA7_qvjNma8tofXbnIbgwQ369dfgXK4or7_IdzFSTHUzU";
        let verifying_key = "MFkwEwYHKoZIzj0CAQYIKoZIzj0DAQcDQgAEaBtE5hCryzaXqsR_HxOn4mJfl54fUwqjXBQV7ACDuRD3blNTjSljOh4IxHrJwzJ_hPE5j5TGfuzU5ucDbN0pfA";
        let client_data_json = "eyJ0eXBlIjoid2ViYXV0aG4uZ2V0IiwiY2hhbGxlbmdlIjoialRyQV95MWJjd0dvczYtSmZ2YTdOQlFKcnNuWlRuTTQ5VlAzUkxNeXB3QXFVVHl6bWJmV0tHVHhCSUN6RnVTMDNFWUpBTldpRTZHUUQ0djNWbmJPOUEiLCJvcmlnaW4iOiJodHRwOi8vbG9jYWxob3N0OjUxNzMiLCJjcm9zc09yaWdpbiI6ZmFsc2UsIm90aGVyX2tleXNfY2FuX2JlX2FkZGVkX2hlcmUiOiJkbyBub3QgY29tcGFyZSBjbGllbnREYXRhSlNPTiBhZ2FpbnN0IGEgdGVtcGxhdGUuIFNlZSBodHRwczovL2dvby5nbC95YWJQZXgifQ";
        let authenticator_data = "SZYN5YgOjGh0NBcPZHZgW4_krrmihjLHmVzzuoMdl2MFAAAAAA";
        let signature = "MEQCID9705cwoCndkpORJoA5mn5jqWXYTYsWigOKG5jmerC_AiADvFzRZMFCrMTQToQdlyC9BiwCEeAQs5Y6bIjiTbKs_g";

        let verifying_key = base64::engine::general_purpose::URL_SAFE_NO_PAD
            .decode(verifying_key)
            .unwrap();

        let verifying_key: VerifyingKey<p256::NistP256> =
            ecdsa::VerifyingKey::from_public_key_der(&verifying_key).unwrap();

        let qq = base64::engine::general_purpose::URL_SAFE_NO_PAD
            .decode(client_data_json)
            .unwrap()
            .to_vec();

        dbg!(&qq);

        // let hasher = Sha256::new();
        let client_data_json_hash = Sha256::digest(
            base64::engine::general_purpose::URL_SAFE_NO_PAD
                .decode(client_data_json)
                .unwrap()
                .to_vec(),
        )
        .to_vec();

        // hasher.update(
        //     base64::engine::general_purpose::URL_SAFE_NO_PAD
        //         .decode(client_data_json)
        //         .unwrap()
        //         .to_vec(),
        // );
        // let client_data_json_hash = hasher.finalize();

        // let client_data_json_hash = ring::digest::digest(
        //     &ring::digest::SHA256,
        //     ,
        // );

        let authenticator_data = base64::engine::general_purpose::URL_SAFE_NO_PAD
            .decode(authenticator_data)
            .unwrap();

        dbg!(&authenticator_data);
        dbg!(&client_data_json_hash);

        let mut message: Vec<u8> = authenticator_data;
        message.extend(&client_data_json_hash);

        dbg!(&message);

        let signature = base64::engine::general_purpose::URL_SAFE_NO_PAD
            .decode(signature)
            .unwrap();

        let signature = Signature::from_bytes(&signature).unwrap();

        let result = verifying_key.verify(&message, &signature);

        dbg!(&result);
    }
}

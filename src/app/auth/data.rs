// pub mod complete;
// pub mod index;
// pub mod join;
// pub mod login;

use axum::Json;
// use passkey::types::webauthn::{
//     AttestationConveyancePreference, AuthenticatorAttestationResponse, CredentialCreationOptions,
//     PublicKeyCredential, PublicKeyCredentialCreationOptions, PublicKeyCredentialParameters,
//     PublicKeyCredentialRpEntity, PublicKeyCredentialType,
// };
use serde::{Deserialize, Serialize};
use serde_with::base64::{Base64, UrlSafe};
use serde_with::formats::{Padded, Unpadded};
use serde_with::serde_as;
use uuid::Uuid;
use validator::Validate;

use super::{entities, User};

#[derive(Deserialize, Validate, Debug)]
pub struct JoinRequestData {
    #[validate(email)]
    pub email: String,
}

// #[serde_as]
// #[derive(Serialize)]
// #[serde(rename_all = "camelCase")]
// pub struct JoinResponseData {
//     #[serde_as(as = "Base64<UrlSafe, Unpadded>")]
//     pub challenge: Vec<u8>,
//     // pub rp: PublicKeyCredentialRpEntity,
//     // pub pub_key_cred_params: Vec<PublicKeyCredentialParameters>,
//     // pub user: JoinUserResponseData,
// }

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub enum JoinResponseData {
    Creation(CredentialCreationOptions),
    Request(CredentialRequestOptions),
}
//     #[serde_as(as = "Base64<UrlSafe, Unpadded>")]
//     pub challenge: Vec<u8>,
//     // pub rp: PublicKeyCredentialRpEntity,
//     // pub pub_key_cred_params: Vec<PublicKeyCredentialParameters>,
//     // pub user: JoinUserResponseData,
// }

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CredentialRequestOptions {
    pub public_key: PublicKeyCredentialRequestOptions,
}

#[serde_as]
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PublicKeyCredentialRequestOptions {
    #[serde_as(as = "Base64<UrlSafe, Unpadded>")]
    pub challenge: Vec<u8>,
    pub rp_id: Option<String>,
    pub allow_credentials: Vec<PublicKeyCredentialDescriptor>,
    // pub user_verification: String,
}

#[serde_as]
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PublicKeyCredentialDescriptor {
    // #[serde_as(as = "Base64<UrlSafe, Unpadded>")]
    pub id: String,
    #[serde(rename = "type")]
    pub tp: String,
    pub transports: Vec<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CredentialCreationOptions {
    pub public_key: PublicKeyCredentialCreationOptions,
}

#[serde_as]
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PublicKeyCredentialCreationOptions {
    #[serde_as(as = "Base64<UrlSafe, Unpadded>")]
    pub challenge: Vec<u8>,
    pub pub_key_cred_params: Vec<PublicKeyCredentialParameters>,
    pub rp: PublicKeyCredentialRpEntity,
    pub user: PublicKeyCredentialUserEntity,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PublicKeyCredentialParameters {
    pub alg: i16,
    #[serde(rename = "type")]
    pub tp: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PublicKeyCredentialRpEntity {
    pub id: Option<String>,
    pub name: String,
}

#[serde_as]
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PublicKeyCredentialUserEntity {
    #[serde_as(as = "Base64<UrlSafe, Unpadded>")]
    pub id: Uuid,
    pub name: String,
    pub display_name: String,
}

#[serde_as]
#[derive(Deserialize, Debug)]
pub struct PublicKeyCredentialWithAttestation {
    pub response: AuthenticatorAttestationResponse,
    pub id: String,
}

#[serde_as]
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AuthenticatorAttestationResponse {
    #[serde(rename = "clientDataJSON")]
    #[serde_as(as = "Base64<UrlSafe, Unpadded>")]
    pub client_data: ClientData,
    pub attestation_object: String,
    pub transports: Vec<String>,
    #[serde_as(as = "Base64<UrlSafe, Unpadded>")]
    pub public_key: Vec<u8>,
    pub public_key_algorithm: i32,
}

#[serde_as]
#[derive(Deserialize, Debug, Serialize)]
pub struct ClientData {
    #[serde(rename = "type")]
    pub tp: String,
    pub challenge: String,
    pub origin: String,
}

impl Into<ClientData> for Vec<u8> {
    fn into(self) -> ClientData {
        serde_json::from_slice::<ClientData>(&self).unwrap()
    }
}

// #[derive(Serialize)]
// #[serde(rename_all = "camelCase")]
// pub struct JoinUserResponseData {
//     pub id: Uuid,
//     pub display_name: String,
//     pub name: String,
// }

// impl From<entities::user_passkey::Model> for JoinResponseData {
//     fn from(user_passkey: entities::user_passkey::Model) -> Self {
//         JoinResponseData {
//             challenge: user_passkey.id,
//             // rp: PublicKeyCredentialRpEntity {
//             //     id: None,
//             //     name: "QQQ".to_string(),
//             // },
//             user: JoinUserResponseData {
//                 id: Uuid::now_v7(),
//                 display_name: "UE_DISPLAY_NAME".into(),
//                 name: "UE_NAME".into(),
//             },
//             // pub_key_cred_params: vec![PublicKeyCredentialParameters {
//             //     ty: PublicKeyCredentialType::PublicKey,
//             //     alg: coset::iana::Algorithm::ES256,
//             // }],
//             // public_key: PublicKeyCredentialCreationOptions {
//             //     rp: PublicKeyCredentialRpEntity {
//             //         id: None,
//             //         name: "QQQ".to_string(),
//             //     },
//             //     user: PublicKeyCredentialUserEntity {
//             //         id: Uuid::now_v7().to_string().as_bytes().to_vec().into(),
//             //         display_name: "UE_DISPLAY_NAME".into(),
//             //         name: "UE_NAME".into(),
//             //     },
//             //     challenge: user_passkey.raw_id.into(),
//             //     pub_key_cred_params: vec![PublicKeyCredentialParameters {
//             //         ty: PublicKeyCredentialType::PublicKey,
//             //         alg: coset::iana::Algorithm::ES256,
//             //     }],
//             //     timeout: None,
//             //     exclude_credentials: None,
//             //     authenticator_selection: None,
//             //     hints: None,
//             //     attestation: AttestationConveyancePreference::None,
//             //     attestation_formats: None,
//             //     extensions: None,
//             // },
//         }
//     }
// }

// impl From<String> for JoinResponseData {
//     fn from(value: String) -> Self {
//         JoinResponseData { ch: value }
//     }
// }

#[derive(Deserialize, Validate, Debug)]
pub struct CompleteRequestData {
    pub first_name: String,
    pub last_name: String,
    pub credential: PublicKeyCredentialWithAttestation,
    // pub client_data_json: String,
    // pub signature: String,
}

#[derive(Serialize)]
pub struct CompleteResponseData {}

#[derive(Deserialize, Validate, Debug)]
pub struct LoginRequestData {
    pub credential: PublicKeyCredentialWithAssertion,
}

#[serde_as]
#[derive(Deserialize, Debug)]
pub struct PublicKeyCredentialWithAssertion {
    pub response: AuthenticatorAssertionResponse,
    pub id: String,
}

#[serde_as]
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AuthenticatorAssertionResponse {
    // #[serde(rename = "clientDataJSON")]
    // #[serde_as(as = "Base64<UrlSafe, Unpadded>")]
    // pub client_data: ClientData,
    #[serde(rename = "clientDataJSON")]
    #[serde_as(as = "Base64<UrlSafe, Unpadded>")]
    pub client_data_json: Vec<u8>,
    #[serde_as(as = "Base64<UrlSafe, Unpadded>")]
    pub signature: Vec<u8>,
    #[serde_as(as = "Base64<UrlSafe, Unpadded>")]
    pub authenticator_data: Vec<u8>,
}

#[derive(Serialize)]
pub struct LoginResponseData {}

#[derive(Serialize)]
pub struct MeResponseData {
    pub user: Option<User>,
}

impl From<Option<User>> for MeResponseData {
    fn from(user: Option<User>) -> Self {
        MeResponseData { user }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_attestation() {
        let data = r#"
        {
            "attestationObject": "o2NmbXRkbm9uZWdhdHRTdG10oGhhdXRoRGF0YVikSZYN5YgOjGh0NBcPZHZgW4_krrmihjLHmVzzuoMdl2NFAAAAALU5dmZIhaprzr_lImKkOaIAIN6dzv5ya5aTM9gj30Bk39NdPorHetvhsMKikd75kp5NpQECAyYgASFYIF4ftXOUKD6Kgm6QDE9tRs_BtsiIXAKlHMX7Zzb44tWfIlggYYxhdnjt6JIjMOty189j5PVr3eiIyEgtUE7HYAkObls",
            "clientDataJSON": "eyJ0eXBlIjoid2ViYXV0aG4uY3JlYXRlIiwiY2hhbGxlbmdlIjoiU3M3YVBibl9WQnJwUzRYNENBY1NzOVZVdVIxMExHV1hSNHFBT0NucU5BcWh6TmxqR0lkbGVsbU1pdGkyc3psRXVrTlpSNEVGb0x0enRtZm5iQ28wZVEiLCJvcmlnaW4iOiJodHRwOi8vbG9jYWxob3N0OjUxNzMiLCJjcm9zc09yaWdpbiI6ZmFsc2V9",
            "publicKey": "MFkwEwYHKoZIzj0CAQYIKoZIzj0DAQcDQgAEXh-1c5QoPoqCbpAMT21Gz8G2yIhcAqUcxftnNvji1Z9hjGF2eO3okiMw63LXz2Pk9Wvd6IjISC1QTsdgCQ5uWw",
            "publicKeyAlgorithm": -7,
            "transports": ["internal"]
        }"#;

        let qq: AuthenticatorAttestationResponse = serde_json::from_str(data).unwrap();

        dbg!(&qq);
    }
}

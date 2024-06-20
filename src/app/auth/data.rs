use serde::{Deserialize, Serialize};
use serde_with::base64::{Base64, UrlSafe};
use serde_with::formats::Unpadded;
use serde_with::serde_as;
use uuid::Uuid;
use validator::Validate;

use super::{entities, User};

#[derive(Deserialize, Validate, Debug)]
pub struct JoinRequestData {
    #[validate(email)]
    pub email: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub enum JoinResponseData {
    Creation(CredentialCreationOptions),
    Request(CredentialRequestOptions),
}

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
}

#[serde_as]
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PublicKeyCredentialDescriptor {
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

#[derive(Deserialize, Validate, Debug)]
pub struct CompleteRequestData {
    pub first_name: String,
    pub last_name: String,
    pub credential: PublicKeyCredentialWithAttestation,
}

#[derive(Serialize)]
pub struct CompleteResponseData {
    pub jwt: String,
}

impl From<(entities::user::Model, String)> for CompleteResponseData {
    fn from((_, jwt): (entities::user::Model, String)) -> Self {
        Self { jwt }
    }
}

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
    #[serde(rename = "clientDataJSON")]
    #[serde_as(as = "Base64<UrlSafe, Unpadded>")]
    pub client_data_json: Vec<u8>,
    #[serde_as(as = "Base64<UrlSafe, Unpadded>")]
    pub signature: Vec<u8>,
    #[serde_as(as = "Base64<UrlSafe, Unpadded>")]
    pub authenticator_data: Vec<u8>,
}

#[derive(Serialize)]
pub struct LoginResponseData {
    pub jwt: String,
}

impl From<(entities::user::Model, String)> for LoginResponseData {
    fn from((_, jwt): (entities::user::Model, String)) -> Self {
        Self { jwt }
    }
}

#[derive(Serialize)]
pub struct MeResponseData {
    pub user: Option<User>,
}

impl From<Option<User>> for MeResponseData {
    fn from(user: Option<User>) -> Self {
        MeResponseData { user }
    }
}

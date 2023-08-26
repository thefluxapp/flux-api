use super::super::entities;
use serde::{Deserialize, Serialize};
use webauthn_rs::prelude::{CreationChallengeResponse, RequestChallengeResponse};

#[derive(Deserialize)]
pub struct RequestData {
    pub email: String,
}

#[derive(Serialize)]
pub struct ResponseData {
    id: String,
    challenge: ChallengeResponse,
}

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
enum ChallengeResponse {
    CreationChallengeResponse(CreationChallengeResponse),
    RequestChallengeResponse(RequestChallengeResponse),
}

impl From<(CreationChallengeResponse, entities::auth_state::Model)> for ResponseData {
    fn from(
        (challenge, auth_state): (CreationChallengeResponse, entities::auth_state::Model),
    ) -> ResponseData {
        ResponseData {
            challenge: ChallengeResponse::CreationChallengeResponse(challenge),
            id: auth_state.id,
        }
    }
}

impl From<(RequestChallengeResponse, entities::auth_state::Model)> for ResponseData {
    fn from(
        (challenge, auth_state): (RequestChallengeResponse, entities::auth_state::Model),
    ) -> ResponseData {
        ResponseData {
            challenge: ChallengeResponse::RequestChallengeResponse(challenge),
            id: auth_state.id,
        }
    }
}

use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct CreateMessagePayload {
    #[validate(length(min = 1))]
    pub text: String,
}

use itertools::Itertools;
use serde::Serialize;
use uuid::Uuid;

use crate::app::auth::User;

use super::entities;

pub mod index;

#[derive(Serialize)]
pub struct IndexResponseData {
    pub streams: Vec<IndexResponseStreamData>,
}

#[derive(Serialize)]
pub struct IndexResponseStreamData {
    pub id: Uuid,
    pub message_id: Uuid,
    pub text: Option<String>,
    pub title: Option<String>,
    pub label: String,
    pub users: Vec<Uuid>,
    pub is_current_user: bool,
    // pub user: Option<ResponseStreamUserData>,
}

impl
    From<(
        Vec<entities::stream::Model>,
        Vec<Vec<entities::stream_user::Model>>,
        Option<User>,
    )> for IndexResponseData
{
    fn from(
        (streams, streams_users, user): (
            Vec<entities::stream::Model>,
            Vec<Vec<entities::stream_user::Model>>,
            Option<User>,
        ),
    ) -> Self {
        Self {
            streams: streams
                .iter()
                .zip(streams_users.iter())
                .map(|(stream, stream_users)| (stream, stream_users, &user).into())
                .collect(),
        }
    }
}

impl
    From<(
        &entities::stream::Model,
        &Vec<entities::stream_user::Model>,
        &Option<User>,
    )> for IndexResponseStreamData
{
    fn from(
        (stream, stream_users, user): (
            &entities::stream::Model,
            &Vec<entities::stream_user::Model>,
            &Option<User>,
        ),
    ) -> Self {
        let label = stream.id.to_string()[stream.id.to_string().len() - 2..].to_uppercase();

        Self {
            id: stream.id,
            message_id: stream.message_id,
            text: stream.text.clone(),
            title: stream.title.clone(),
            users: stream_users
                .iter()
                .map(|stream_user| stream_user.user_id)
                .collect(),
            label,
            is_current_user: match user {
                Some(user) => stream_users.iter().map(|x| x.user_id).contains(&user.id),
                _ => false,
            },
        }
    }
}

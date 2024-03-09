use sea_orm::DbConn;

use crate::app::AppError;

use super::{entities, repo};

// pub async fn find_user_streams(
//     db: &DbConn,
//     user: User,
// ) -> Result<Vec<entities::stream::Model>, AppError> {
//     let streams = repo::find_user_streams(db, user.id).await?;

//     Ok(streams)
// }

pub async fn find_all_streams(
    db: &DbConn,
) -> Result<
    (
        Vec<entities::stream::Model>,
        Vec<Vec<entities::stream_user::Model>>,
    ),
    AppError,
> {
    let (streams, streams_users) = repo::find_all_streams_with_users(db).await?;

    Ok((streams, streams_users))
}

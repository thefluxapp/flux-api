use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, Set, TransactionTrait};
use tracing::info;
use validator::Validate;

use super::{
    data::{CreateData, IndexData},
    entities,
    payload::CreateMessagePayload,
};
use crate::app::{streams::service::StreamsService, User};

pub struct MessagesService {}

impl MessagesService {}

#[cfg(test)]
mod tests {}

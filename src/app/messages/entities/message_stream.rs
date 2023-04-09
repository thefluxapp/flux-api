use chrono::Utc;
use sea_orm::{entity::prelude::*, Set};
use serde::Serialize;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize)]
#[sea_orm(table_name = "messages_streams")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub message_id: Uuid,
    pub stream_id: Uuid,
    pub created_at: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::message::Entity",
        from = "Column::MessageId",
        to = "super::message::Column::Id"
    )]
    Message,
    #[sea_orm(
        belongs_to = "super::stream::Entity",
        from = "Column::StreamId",
        to = "super::stream::Column::Id"
    )]
    Stream,
}

impl Related<super::stream::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Stream.def()
    }
}

impl Related<super::message::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Message.def()
    }
}

// TODO: DRY for all models
#[async_trait::async_trait]
impl ActiveModelBehavior for ActiveModel {
    async fn before_save<C>(mut self, _: &C, insert: bool) -> Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        if self.is_not_set(Column::Id) && insert {
            self.id = Set(Uuid::now_v7());
            self.created_at = Set(Utc::now());
        }

        Ok(self)
    }
}

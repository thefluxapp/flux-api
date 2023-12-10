use chrono::Utc;
use sea_orm::{entity::prelude::*, Set};
use serde::Serialize;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize)]
#[sea_orm(table_name = "messages")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub text: String,
    pub user_id: Uuid,
    pub created_at: DateTime,
}

impl Model {
    pub fn status(&self) -> String {
        String::from("active")
    }

    pub fn order(&self) -> i64 {
        self.created_at.timestamp_micros()
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::UserId",
        to = "super::user::Column::Id"
    )]
    User,
    #[sea_orm(has_one = "super::message_stream::Entity")]
    MessageStream,
    #[sea_orm(has_one = "super::stream::Entity")]
    Stream,
}

impl Related<super::message_stream::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::MessageStream.def()
    }
}

impl Related<super::stream::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Stream.def()
    }
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
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
            self.created_at = Set(Utc::now().naive_utc());
        }

        Ok(self)
    }
}

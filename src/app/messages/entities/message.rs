use sea_orm::{entity::prelude::*, Set};
use serde::Serialize;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize)]
#[sea_orm(table_name = "messages")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub text: String,
    pub user_id: Uuid,
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

// TODO: DRY for all models
#[async_trait::async_trait]
impl ActiveModelBehavior for ActiveModel {
    async fn before_save<C>(mut self, _: &C, insert: bool) -> Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        if self.is_not_set(Column::Id) && insert {
            self.id = Set(Uuid::now_v7());
        }

        Ok(self)
    }
}

use sea_orm::entity::prelude::*;
use serde::Serialize;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize)]
#[sea_orm(table_name = "streams")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub title: Option<String>,
    pub user_id: Option<Uuid>,
    pub message_id: Option<Uuid>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::UserId",
        to = "super::user::Column::Id"
    )]
    User,
    #[sea_orm(has_many = "super::message_stream::Entity")]
    MessageStream,
}

// impl Related<super::message_stream::Entity> for Entity {
//     fn to() -> RelationDef {
//         Relation::MessageStream.def()
//     }
// }

impl ActiveModelBehavior for ActiveModel {}

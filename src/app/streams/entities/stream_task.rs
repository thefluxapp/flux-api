use sea_orm::entity::prelude::*;
use serde::Serialize;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize)]
#[sea_orm(table_name = "stream_tasks")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub stream_id: Uuid,
    pub created_at: DateTime,
    pub processed_at: Option<DateTime>,
    pub started_at: Option<DateTime>,
    pub started_by: Option<Uuid>,
    pub failed_at: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
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

impl ActiveModelBehavior for ActiveModel {}

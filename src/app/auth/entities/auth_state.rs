use chrono::Utc;
use rand::distributions::{Alphanumeric, DistString};
use sea_orm::{entity::prelude::*, Set};
use serde::Serialize;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize)]
#[sea_orm(table_name = "auth_states")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: String,
    // #[sea_orm(column_type = "JsonBinary")]
    pub state: Json,
    pub user_id: Uuid,
    pub email: String,
    pub created_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

#[async_trait::async_trait]
impl ActiveModelBehavior for ActiveModel {
    async fn before_save<C>(mut self, _: &C, insert: bool) -> Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        if self.is_not_set(Column::Id) && insert {
            self.id = Set(Alphanumeric.sample_string(&mut rand::thread_rng(), 128));
            self.created_at = Set(Utc::now().naive_utc());
        }

        Ok(self)
    }
}

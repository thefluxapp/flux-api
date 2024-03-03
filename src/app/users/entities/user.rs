use chrono::Utc;
use sea_orm::{entity::prelude::*, Set};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub email: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    #[sea_orm(column_type = "JsonBinary")]
    pub passkeys: Json,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

impl Model {
    pub fn name(&self) -> String {
        self.first_name.as_ref().unwrap_or(&self.email).clone()
    }

    pub fn image(&self) -> Option<String> {
        None
    }
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        panic!("No RelationDef")
    }
}

#[async_trait::async_trait]
impl ActiveModelBehavior for ActiveModel {
    async fn before_save<C>(mut self, _: &C, insert: bool) -> Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        if insert {
            self.created_at = Set(Utc::now().naive_utc());
        }
        self.updated_at = Set(Utc::now().naive_utc());

        Ok(self)
    }
}

use chrono::{NaiveDate, NaiveDateTime};
use sea_orm::entity::prelude::*;
use sea_orm::{ActiveModelBehavior, ActiveModelTrait, EnumIter};
use serde::{Deserialize, Serialize};

#[sea_orm::model]
#[derive(Clone, Debug, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub avatar: Option<String>,
    pub first_name: String,
    pub last_name: String,
    pub username: String,
    pub email: String,
    pub password: Option<String>,
    pub birth_of_date: Option<NaiveDate>,
    #[sea_orm(has_many)]
    pub address: HasMany<super::super::address::address::Entity>,
    pub phone_number: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub deleted_at: Option<NaiveDateTime>,
}

impl ActiveModelBehavior for ActiveModel {}
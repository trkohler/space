use async_graphql::*;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub email: String,
    pub display_name: String,
    pub created_at: DateTime,
    pub updated_at: Option<DateTime>,
    pub oauth_provider: OauthProvider,
    pub role: Role,
}

#[derive(EnumIter, DeriveActiveEnum, Debug, PartialEq, Serialize, Deserialize, Clone)]
#[sea_orm(rs_type = "String", db_type = "String(Some(1))")]
pub enum OauthProvider {
    #[sea_orm(string_value = "G")]
    Google,
}

#[derive(
    EnumIter, DeriveActiveEnum, Debug, PartialEq, Serialize, Deserialize, Clone, Enum, Eq, Copy,
)]
#[sea_orm(rs_type = "String", db_type = "String(Some(2))")]
pub enum Role {
    #[sea_orm(string_value = "SA")]
    SpaceAdmin,
    #[sea_orm(string_value = "SU")]
    SpaceUser,
    #[sea_orm(string_value = "AA")]
    ApplicationAdmin,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

impl Entity {
    pub fn find_by_id(id: i32) -> Select<Entity> {
        Self::find().filter(Column::Id.eq(id))
    }

    pub fn find_by_email(email: String) -> Select<Entity> {
        Self::find().filter(Column::Email.eq(email))
    }
}

#[derive(SimpleObject, Debug)]
pub struct UserNode {
    pub id: i32,
    pub email: String,
    pub display_name: String,
    pub role: Role,
}

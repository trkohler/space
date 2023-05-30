use async_graphql::*;
use sea_orm::{entity::prelude::*, DeleteMany, JsonValue};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, SimpleObject)]
#[sea_orm(table_name = "maps")] // TODO: Change to "plans"
#[graphql(concrete(name = "Map", params()))] // TODO: Change to "Plan"
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    #[graphql(skip_output)]
    pub image: Vec<u8>,
    pub coordinates: Coordinate,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, FromJsonQueryResult, SimpleObject)]
pub struct Coordinate {
    pub x: u32,
    pub y: u32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::bookable_resource::Entity")]
    BookableResource,
}

impl Related<super::bookable_resource::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::BookableResource.def()
    }
}

impl Entity {
    pub fn find_by_id(id: i32) -> Select<Entity> {
        Self::find().filter(Column::Id.eq(id))
    }
}

impl ActiveModelBehavior for ActiveModel {}
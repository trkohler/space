use async_graphql::*;
use sea_orm::{entity::prelude::*, DeleteMany, JsonValue};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, SimpleObject)]
#[sea_orm(table_name = "maps")]
#[graphql(concrete(name = "Map", params()))]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    #[graphql(skip_output)]
    pub image: Vec<u8>,
    pub coordinates: Coordinate,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, FromJsonQueryResult, SimpleObject)]
pub struct Coordinate {
    x: u32,
    y: u32,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        panic!("No RelationDef")
    }
}

impl Entity {
    pub fn find_by_id(id: i32) -> Select<Entity> {
        Self::find().filter(Column::Id.eq(id))
    }
}

impl ActiveModelBehavior for ActiveModel {}
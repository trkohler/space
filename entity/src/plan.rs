use crate::bookable_resource;
use crate::bookable_resource::ResourceNode;
use async_graphql::*;
use sea_orm::{entity::prelude::*, SelectTwoMany};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "plans")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub image: Vec<u8>,
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

    pub fn find_with_resources(id: i32) -> SelectTwoMany<Entity, bookable_resource::Entity> {
        Self::find()
            .filter(Column::Id.eq(id))
            .find_with_related(bookable_resource::Entity)
    }
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(SimpleObject)]
pub struct PlanNode {
    pub id: i32,
    pub resources: Vec<ResourceNode>,
}

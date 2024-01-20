use async_graphql::*;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, SimpleObject)]
#[sea_orm(table_name = "bookable_resources")]
#[graphql(concrete(name = "BookableResource", params()))]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub coordinate: Coordinate,
    pub plan_id: i32,
    pub kind: BookableResourceKind,
    pub qr_code: Option<String>,
}

#[derive(
    Copy, Clone, Debug, Eq, PartialEq, EnumIter, Serialize, Deserialize, DeriveActiveEnum, Enum,
)]
#[sea_orm(rs_type = "i32", db_type = "Integer")]
pub enum BookableResourceKind {
    #[sea_orm(num_value = 0)]
    Workspace,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, FromJsonQueryResult, SimpleObject)]
pub struct Coordinate {
    pub x: f32,
    pub y: f32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::plan::Entity",
        from = "Column::PlanId",
        to = "super::plan::Column::Id"
    )]
    Plan,
}

impl Related<super::plan::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Plan.def()
    }
}

impl Entity {
    pub fn find_by_id(id: i32) -> Select<Entity> {
        Self::find().filter(Column::Id.eq(id))
    }

    pub fn find_by_coordinates(plan_id: i32, coordinate: Coordinate) -> Select<Entity> {
        Self::find()
            .filter(Column::PlanId.eq(plan_id))
            .filter(Column::Coordinate.eq(coordinate))
    }
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(SimpleObject)]
pub struct ResourceNode {
    pub id: i32,
    pub coordinate: Coordinate,
    pub qr_code: Option<String>,
    pub kind: BookableResourceKind,
}

use async_graphql::{Context, Object, Result};
use entity::async_graphql::{self, InputObject};
use entity::bookable_resource::*;
use graphql_example_service::Mutation;

use crate::db::Database;

#[derive(InputObject)]
pub struct CreateResource {
    pub x: f32,
    pub y: f32,
    pub kind: BookableResourceKind,
    pub plan_id: i32,
}

#[derive(InputObject)]
pub struct DeleteResource {
    pub x: f32,
    pub y: f32,
    pub plan_id: i32,
}

#[derive(Default)]
pub struct ResourceMutation;

#[Object]
impl ResourceMutation {
    pub async fn create_resource(
        &self,
        ctx: &Context<'_>,
        input: CreateResource,
    ) -> Result<ResourceNode> {
        let db = ctx.data::<Database>().unwrap();
        let conn = db.get_connection();
        let coordinates = Coordinate {
            x: input.x,
            y: input.y,
        };

        let result = Mutation::create_bookable_resource(conn, coordinates, input.plan_id).await?;
        Ok(result)
    }

    pub async fn remove_resource(&self, ctx: &Context<'_>, input: DeleteResource) -> Result<bool> {
        let db = ctx.data::<Database>().unwrap();
        let conn = db.get_connection();
        // removal by coordinates
        let result = Mutation::remove_bookable_resource(
            conn,
            input.plan_id,
            Coordinate {
                x: input.x,
                y: input.y,
            },
        )
        .await?;
        Ok(result.rows_affected > 0)
    }
}

use crate::async_graphql::futures_util::AsyncReadExt;
use crate::async_graphql::Upload;
use async_graphql::{Context, Object, Result};
use entity::async_graphql::{self, InputObject, SimpleObject};
use entity::plan;
use graphql_example_service::Mutation;
use std::io::Read;

use crate::db::Database;

#[derive(Default)]
pub struct PlanMutation;

#[Object]
impl PlanMutation {
    pub async fn create_plan(
        &self,
        ctx: &Context<'_>,
        file: Upload,
    ) -> Result<plan::PlanNode> {
        let db = ctx.data::<Database>().unwrap();
        let conn = db.get_connection();
        let converted_file: Result<Vec<_>, _> =
            file.value(ctx).unwrap().content.bytes().collect();
        let converted_file = converted_file.expect("Cannot convert file");

        // Ok(Mutation::create_note(conn, input.into_model_with_arbitrary_id()).await?)
        let result = Mutation::create_plan(conn, converted_file).await?;
        Ok(result)
    }
}

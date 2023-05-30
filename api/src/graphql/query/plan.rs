use async_graphql::{Context, Object, Result};
use entity::{async_graphql, plan, note};
use graphql_example_service::Query;

use crate::db::Database;

#[derive(Default)]
pub struct MapQuery;

#[Object]
impl MapQuery {
    async fn plan(&self, ctx: &Context<'_>, id: i32) -> Result<Option<plan::Model>> {
        let db = ctx.data::<Database>().unwrap();
        let conn = db.get_connection();

        Ok(Query::find_map_by_id(conn, id)
            .await
            .map_err(|e| e.to_string())?)
    }
}

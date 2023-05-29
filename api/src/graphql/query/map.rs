use async_graphql::{Context, Object, Result};
use entity::{async_graphql, note};
use graphql_example_service::Query;

use crate::db::Database;

#[derive(Default)]
pub struct MapQuery;

#[Object]
impl MapQuery {

    async fn map(&self, ctx: &Context<'_>) -> Option<String> {
        None
    }
}
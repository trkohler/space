use async_graphql::{EmptySubscription, Schema};
use entity::async_graphql;
use log::error;
use migration::{Migrator, MigratorTrait};

use crate::{
    db::Database,
    graphql::{mutation::Mutation, query::Query},
};

pub type AppSchema = Schema<Query, Mutation, EmptySubscription>;

/// Builds the GraphQL Schema, attaching the Database to the context
pub async fn build_schema(connection_string: &String) -> AppSchema {
    let db = Database::new(connection_string).await;

    let _ = Migrator::up(db.get_connection(), None).await.map_err(|_| {
        error!("Error running migrations");
    });

    Schema::build(Query::default(), Mutation::default(), EmptySubscription)
        .data(db)
        .finish()
}

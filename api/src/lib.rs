mod db;
mod graphql;

use entity::async_graphql;

use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::extract::State;
pub use axum::{
    extract::Extension,
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
pub use graphql::schema::{build_schema, AppSchema};
use shuttle_secrets::SecretStore;
pub use tower_http::cors::CorsLayer;

pub async fn graphql_handler(
    schema: Extension<AppSchema>,
    State(state): State<AppState>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    schema.execute(req.into_inner().data(state)).await.into()
}

pub async fn graphql_playground() -> impl IntoResponse {
    Html(playground_source(GraphQLPlaygroundConfig::new(
        "/api/graphql",
    )))
}

#[derive(Clone)]
pub struct AppState {
    pub secrets: SecretStore,
}

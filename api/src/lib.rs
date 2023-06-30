mod db;
mod graphql;
pub mod guard;

use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::extract::{FromRef, State};
use axum::http::StatusCode;
pub use axum::{
    extract::Extension,
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
pub use axum_extra::extract::cookie::{Key, SignedCookieJar};
use entity::async_graphql;
pub use graphql::schema::{build_schema, AppSchema};
pub use guard::guard;
use shuttle_secrets::SecretStore;
use std::sync::{Arc, Mutex};
pub use tower_cookies::CookieManagerLayer;
pub use tower_http::cors::CorsLayer;

pub async fn graphql_handler(
    schema: Extension<AppSchema>,
    secret_store: Extension<SecretStore>,
    parsed_token: Extension<Option<guard::ParsedGoogleToken>>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    println!("Parsed token in handler: {:?}", parsed_token.0);

    let execute = schema
        .execute(req.into_inner().data(secret_store.0).data(parsed_token.0))
        .await;

    execute.into()
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



#![feature(iterator_try_collect)]

mod db;
mod graphql;

use entity::async_graphql;

use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    extract::Extension,
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use axum::http::Method;
use graphql::schema::{build_schema, AppSchema};

#[cfg(debug_assertions)]
use dotenvy::dotenv;
use tower_http::cors::CorsLayer;

async fn graphql_handler(schema: Extension<AppSchema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

async fn graphql_playground() -> impl IntoResponse {
    Html(playground_source(GraphQLPlaygroundConfig::new(
        "/api/graphql",
    )))
}



#[tokio::main]
pub async fn main() {
    #[cfg(debug_assertions)]
    dotenv().ok();

    let schema = build_schema().await;

    // let cors = CorsLayer::new()
    //     // allow `GET` and `POST` when accessing the resource
    //     .allow_methods(vec![Method::GET, Method::POST]);

    let app = Router::new()
        .route(
            "/api/graphql",
            get(graphql_playground).post(graphql_handler),
        )
        .layer(CorsLayer::permissive())
        .layer(Extension(schema));

    println!("Playground: http://localhost:8080/api/graphql");

    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

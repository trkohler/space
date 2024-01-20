use anyhow::anyhow;
use axum::http::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use axum::http::Method;
use axum::middleware;
use graphql_api::{
    build_schema, get, graphql_handler, graphql_playground, guard, CookieManagerLayer, CorsLayer,
    Extension, Router,
};

use shuttle_secrets::SecretStore;

#[shuttle_runtime::main]
async fn axum(#[shuttle_secrets::Secrets] secret_store: SecretStore) -> shuttle_axum::ShuttleAxum {
    let connection_string = secret_store.get("DATABASE_URL").ok_or(anyhow!(
        "DATABASE_URL not found in secrets. Did you forget to set it?"
    ))?;

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(["http://localhost:3000".parse().unwrap()])
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE])
        .allow_credentials(true);

    let schema = build_schema(&connection_string).await;

    let router = Router::new()
        .route(
            "/api/graphql",
            get(graphql_playground).post(graphql_handler),
        )
        .layer(Extension(schema))
        .layer(middleware::from_fn(guard))
        .layer(cors)
        .layer(Extension(secret_store))
        .layer(Extension(None::<guard::ParsedGoogleToken>))
        .layer(CookieManagerLayer::new());

    Ok(router.into())
}

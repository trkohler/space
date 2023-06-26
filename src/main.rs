use graphql_api::{
    build_schema, get, graphql_handler, graphql_playground, AppState, CorsLayer, Extension, Router,
};
use shuttle_secrets::SecretStore;

#[shuttle_runtime::main]
async fn axum(#[shuttle_secrets::Secrets] secret_store: SecretStore) -> shuttle_axum::ShuttleAxum {
    let connection_string = secret_store.get("DATABASE_URL").unwrap();

    let schema = build_schema(&connection_string).await;
    let state = AppState {
        secrets: secret_store,
    };

    let router = Router::new()
        .route(
            "/api/graphql",
            get(graphql_playground).post(graphql_handler),
        )
        .layer(CorsLayer::permissive())
        .layer(Extension(schema))
        .with_state(state);

    Ok(router.into())
}

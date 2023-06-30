use crate::{SecretStore, StatusCode};
use axum::headers::authorization::Bearer;
use axum::headers::{Authorization, HeaderMapExt};
use axum::http::Request;
use axum::middleware::Next;
use axum::response::Response;
use google_oauth::{AsyncClient, GooglePayload};

#[derive(Debug, Clone)]
pub struct ParsedGoogleToken {
    pub email: Option<String>,
    pub email_verified: Option<bool>,
    pub given_name: Option<String>,
    pub family_name: Option<String>,
    pub picture: Option<String>,
}

pub async fn guard<T>(mut request: Request<T>, next: Next<T>) -> Result<Response, StatusCode> {
    let secret_store = request.extensions().get::<SecretStore>().ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;
    let token = request
        .headers()
        .typed_get::<Authorization<Bearer>>()
        .map(|header| header.token().to_owned());

    let secret = secret_store.get("GOOGLE_CLIENT_ID").ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;

    if let Some(token) = token {
        let client = AsyncClient::new(secret);
        let data = client
            .validate_id_token(token)
            .await
            .map_err(|_| StatusCode::UNAUTHORIZED)?;
        let GooglePayload {
            email,
            email_verified,
            given_name,
            family_name,
            picture,
            ..
        } = data.to_owned();

        let parsed_token = ParsedGoogleToken {
            email,
            email_verified,
            given_name,
            family_name,
            picture,
        };

        request.extensions_mut().insert(Some(parsed_token));
    }

    Ok(next.run(request).await)
}

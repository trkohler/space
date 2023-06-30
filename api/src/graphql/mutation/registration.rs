use crate::db::Database;
use async_graphql::{Context, Object, Result, InputObject, Error as GraphQLError, ErrorExtensions};
use axum_extra::extract::cookie::Cookie;
use cookie::time::Duration;

use crate::{AppState, SecretStore};
use entity::sea_orm::DbErr;
use entity::user::UserNode;
use google_oauth::{AsyncClient, GooglePayload};
use graphql_example_service::{Mutation, Query};
use thiserror::Error;

#[derive(Default)]
pub struct RegistrationMutation;

#[derive(InputObject, Debug)]
pub struct UserInput {
    pub google_jwt_token: String,
}

#[derive(Error, Debug)]
pub enum RegisterUserError {
    #[error("invalid data provided by oauth provider. Missing field: {missing_field:?}")]
    InvalidDataProvided { missing_field: String },

    #[error("google error: {0}")]
    GoogleError(String),

    #[error("Database error: {0}")]
    DbError(#[from] DbErr),
}

impl ErrorExtensions for RegisterUserError {
    fn extend(&self) -> GraphQLError {
        GraphQLError::new(format!("{:?}", self)).extend_with(|err, e| {
            e.set("code", 500);
        })
    }
}

#[Object]
impl RegistrationMutation {
    pub async fn register(
        &self,
        ctx: &Context<'_>,
        user_input: UserInput,
    ) -> Result<UserNode, RegisterUserError> {

        let db = ctx.data::<Database>().unwrap();

        let client_id = ctx
            .data::<SecretStore>()
            .unwrap()
            .get("GOOGLE_CLIENT_ID")
            .unwrap();

        let client = AsyncClient::new(client_id);

        let data = client
            .validate_id_token(user_input.google_jwt_token.clone())
            .await;
        match data {
            Ok(data) => {
                let GooglePayload {
                    email,
                    family_name,
                    given_name,
                    exp,
                    ..
                } = data;

                let email = email.ok_or(RegisterUserError::InvalidDataProvided {
                    missing_field: "email".to_owned(),
                })?;
                let family_name = family_name.ok_or(RegisterUserError::InvalidDataProvided {
                    missing_field: "family_name".to_owned(),
                })?;
                let given_name = given_name.ok_or(RegisterUserError::InvalidDataProvided {
                    missing_field: "given_name".to_owned(),
                })?;
                let display_name = given_name.clone() + " " + &family_name;

                let expiring = Duration::from(Duration::seconds(exp as i64));

                let node = Mutation::register_user(db.get_connection(), email, display_name)
                    .await
                    .map_err(|e| RegisterUserError::DbError(e))?;

                let cookie: Cookie =
                    Cookie::build("google_token", user_input.google_jwt_token.clone())
                        .path("/")
                        .secure(true)
                        .same_site(cookie::SameSite::None)
                        .http_only(true)
                        .max_age(expiring)
                        .finish();

                ctx.insert_http_header("Set-Cookie", cookie.to_string());

                Ok(node)
            }
            Err(e) => Err(RegisterUserError::GoogleError(e.to_string())),
        }
    }
}

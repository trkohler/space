use crate::async_graphql::{Error as GraphQLError, ErrorExtensions};
use crate::db::Database;
use async_graphql::{Context, Object, Result};
use entity::async_graphql::{self, InputObject};

use crate::AppState;
use entity::sea_orm::DbErr;
use entity::user::UserNode;
use google_oauth::{AsyncClient, GooglePayload};
use graphql_example_service::Mutation;
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

        let client_id = ctx
            .data::<AppState>()
            .unwrap()
            .secrets
            .get("GOOGLE_CLIENT_ID")
            .unwrap();

        let client = AsyncClient::new(client_id);
        let data = client.validate_id_token(user_input.google_jwt_token).await;
        match data {
            Ok(data) => {
                let GooglePayload {
                    email,
                    family_name,
                    given_name,
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

                let db = ctx.data::<Database>().unwrap();
                let node = Mutation::register_user(db.get_connection(), email, display_name)
                    .await
                    .map_err(|e| RegisterUserError::DbError(e))?;
                Ok(node)
            }
            Err(e) => Err(RegisterUserError::GoogleError(e.to_string())),
        }
    }
}

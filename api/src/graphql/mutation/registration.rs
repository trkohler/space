use crate::db::Database;
use async_graphql::{Context, InputObject, Object, Result};

use crate::SecretStore;
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
    GoogleError(#[from] anyhow::Error),

    #[error("Database error: {0}")]
    DbError(#[from] DbErr),

    #[error("Failed to extract data from context")]
    ExtractContextError,

    #[error("No secret provided")]
    SecretIsMissing,
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
            .map_err(|_| RegisterUserError::ExtractContextError)?
            .get("GOOGLE_CLIENT_ID")
            .ok_or(RegisterUserError::SecretIsMissing)?;

        let client = AsyncClient::new(client_id);

        let data = client
            .validate_id_token(user_input.google_jwt_token.clone())
            .await
            .map_err(RegisterUserError::GoogleError)?;

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
        let display_name = format!("{} {}", given_name, family_name);

        let node = Mutation::register_user(db.get_connection(), email, display_name)
            .await
            .map_err(RegisterUserError::DbError)?;

        Ok(node)
    }
}

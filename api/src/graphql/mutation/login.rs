use crate::db::Database;
use crate::guard;
use async_graphql::{Context, Object, Result};
use entity::user::UserNode;
use graphql_example_service::sea_orm::DbErr;
use graphql_example_service::{Mutation, Query};
use thiserror::Error;

#[derive(Default)]
pub struct LoginMutation;

#[derive(Error, Debug)]
pub enum LoginError {
    #[error("Failed to extract token")]
    ExtractTokenError,

    #[error("Failed to extract db connection")]
    ExtractDbError,

    #[error("No auth token provided")]
    NoAuthToken,

    #[error("Token must have {missing_field} field")]
    TokenError { missing_field: String },

    #[error("Database error: {0}")]
    DbError(#[from] DbErr),
}


#[Object]
impl LoginMutation {
    pub async fn login(&self, ctx: &Context<'_>) -> Result<UserNode> {
        let parsed_token = ctx
            .data::<Option<guard::ParsedGoogleToken>>()
            .map_err(|_| LoginError::ExtractTokenError)?
            .as_ref()
            .ok_or(LoginError::NoAuthToken)?;

        let db = ctx
            .data::<Database>()
            .map_err(|_| LoginError::ExtractDbError)?;

        let email = parsed_token
            .email
            .to_owned()
            .ok_or(LoginError::TokenError {
                missing_field: "email".to_string(),
            })?;

        let user = Query::get_user_by_email(db.get_connection(), email.clone())
            .await
            .map_err(|e| LoginError::DbError(e))?;
        if let Some(user) = user {
            Ok(user)
        } else {
            let family_name =
                parsed_token
                    .family_name
                    .to_owned()
                    .ok_or(LoginError::TokenError {
                        missing_field: "family_name".to_string(),
                    })?;
            let given_name = parsed_token
                .given_name
                .to_owned()
                .ok_or(LoginError::TokenError {
                    missing_field: "given_name".to_string(),
                })?;
            let display_name = format!("{} {}", given_name, family_name);

            let node = Mutation::register_user(db.get_connection(), email.clone(), display_name)
                .await
                .map_err(|e| LoginError::DbError(e))?;

            Ok(node)
        }
    }
}

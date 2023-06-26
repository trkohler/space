use crate::async_graphql::futures_util::AsyncReadExt;
use crate::async_graphql::{ErrorExtensions, Upload};
use crate::db::Database;
use async_graphql::{Context, Object, Result};
use entity::async_graphql::{self, InputObject, SimpleObject};

use crate::AppState;
use axum::extract::State;
use google_oauth::AsyncClient;
use graphql_example_service::Mutation;
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use std::io::Read;

#[derive(Default)]
pub struct RegistrationMutation;

#[derive(InputObject, Debug)]
pub struct UserInput {
    pub google_jwt_token: String,
}

#[Object]
impl RegistrationMutation {
    pub async fn register(&self, ctx: &Context<'_>, user_input: UserInput) -> Result<&str> {
        println!("registering user with input {:?}", user_input);
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
                println!("data {:?}", data);
                Ok("success")
            }
            Err(e) => Err(e.extend_with(|_, e| e.set("details", "Invalid token"))),
        }
    }
}

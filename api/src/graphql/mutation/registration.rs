use crate::async_graphql::futures_util::AsyncReadExt;
use crate::async_graphql::{ErrorExtensions, Upload};
use crate::db::Database;
use async_graphql::{Context, Object, Result};
use entity::async_graphql::{self, InputObject, SimpleObject};
use entity::plan;

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
    pub client_id: String,
}
#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    iss: String,
    aud: String,
    azp: String,
    email: String,
    email_verified: bool,
    exp: i64,
    family_name: String,
    given_name: String,
    iat: i64,
    jti: String,
    name: String,
    nbf: i64,
    picture: String,
}

#[Object]
impl RegistrationMutation {
    pub async fn register(&self, ctx: &Context<'_>, user_input: UserInput) -> Result<&str> {
        println!("registering user with input {:?}", user_input);

        let client = AsyncClient::new(
            "612437101924-59mhs9gv5j3m0lhcrq97pj7tjhmnm4b7.apps.googleusercontent.com",
        );
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

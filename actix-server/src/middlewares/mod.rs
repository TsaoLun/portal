use actix_web::http::header::HeaderMap;
use async_graphql::{Context, ErrorExtensions, FieldError, Guard, Result};
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use std::env;

pub const EXP: i64 = 7 * 24 * 60 * 60;

#[derive(Debug, Serialize, Deserialize)]
pub struct Token {
    pub token: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub user: String,
    pub exp: i64,
    pub iat: i64,
}
pub struct TokenGuard;

#[async_trait::async_trait]
impl Guard for TokenGuard {
    async fn check(&self, ctx: &Context<'_>) -> Result<(), FieldError> {
        let token = ctx.data_opt::<Token>();
        if let Some(token) = token {
            match decode::<Claims>(
                &token.token,
                &DecodingKey::from_secret(env::var("PORTAL_JWT_KEY").unwrap().as_bytes()),
                &Validation::new(Algorithm::HS512),
            ) {
                Ok(_e) => Ok(()),
                Err(e) => {
                    println!("{:?}", e);
                    Err(FieldError::from("登录过期，请重新登陆!"))
                        .map_err(|err| err.extend_with(|_, e| e.set("code", "EXPIRED_TOKEN")))
                }
            }
        } else {
            Err(FieldError::from("无效 Token"))
                .map_err(|err| err.extend_with(|_, e| e.set("code", "INVALID_TOKEN")))
        }
    }
}

pub fn get_token_from_headers(headers: &HeaderMap) -> Option<Token> {
    headers.get("Authorization").and_then(|value| {
        value
            .to_str()
            .map(|s| Token {
                token: s.to_string().replace("Bearer ", ""),
            })
            .ok()
    })
}

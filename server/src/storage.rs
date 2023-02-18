use async_graphql::{Context, EmptySubscription, FieldError, FieldResult, Object, Schema};
use chrono::Utc;
use futures_util::lock::Mutex;
use jsonwebtoken::{
    decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation,
};
use serde::{Deserialize, Serialize};
use std::{env, sync::Arc};

pub type DataSchema = Schema<Query, Mutation, EmptySubscription>;
use slab::Slab;
pub struct Query;

pub type Storage = Arc<Mutex<Slab<String>>>;

#[derive(Debug, Serialize, Deserialize)]
pub struct Token {
    pub token: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    user: String,
    exp: i64,
    iat: i64,
}

#[Object]
impl Query {
    async fn get(&self, ctx: &Context<'_>) -> FieldResult<String> {
        validate(ctx)?;
        let data = ctx.data_unchecked::<Storage>().lock().await;
        if data.is_empty() {
            Ok("".to_string())
        } else {
            Ok(data[0].clone())
        }
    }
    async fn login(&self, ctx: &Context<'_>, username: String, password: String) -> FieldResult<TokenResponse> {
        if username == env::var("USERNAME").unwrap() && password == env::var("PASSWORD").unwrap() {
            let now = Utc::now().timestamp_millis();
            let claims = Claims {
                user: username,
                exp: now + 180000,
                iat: now,
            };
            let key = env::var("JWT_KEY").unwrap();
            let key = key.as_bytes();
            let header = Header {
                alg: Algorithm::HS512,
                ..Default::default()
            };
            let token = match encode(&header, &claims, &EncodingKey::from_secret(key)) {
                Ok(t) => t,
                Err(e) => {
                    println!("{}", e);
                    Err(FieldError::from("unknown err"))?
                },
            };
            Ok(TokenResponse {
                ok: true,
                token: Some(token),
            })
        } else {
            Ok(TokenResponse {
                ok: false,
                token: None,
            })
        }
    }
}

pub struct Mutation;

#[Object]
impl Mutation {
    async fn set(&self, ctx: &Context<'_>, data: String) -> FieldResult<bool> {
        validate(ctx)?;
        let mut storage = ctx.data_unchecked::<Storage>().lock().await;
        storage.clear();
        storage.insert(data);
        Ok(true)
    }
}

#[derive(Clone)]
pub struct TokenResponse {
    ok: bool,
    token: Option<String>,
}

#[Object]
impl TokenResponse {
    async fn ok(&self) -> bool {
        self.ok
    }
    async fn token(&self) -> Option<String> {
        self.token.clone()
    }
}

fn validate(ctx: &Context<'_>) -> Result<(), FieldError> {
    let token = ctx.data_opt::<Token>();
    if let Some(token) = token {
        println!("{:?}", token);
        match decode::<Claims>(
            &token.token,
            &DecodingKey::from_secret(env::var("JWT_KEY").unwrap().as_bytes()),
            &Validation::new(Algorithm::HS512),
        ) {
            Ok(_) => Ok(()),
            Err(e) => {
                println!("{:?}", e);
                Err(FieldError::from(format!("InvalidToken")))
            }
        }
    } else {
        Err(FieldError::from(format!("InvalidFormat")))
    }
}

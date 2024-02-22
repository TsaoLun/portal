use async_graphql::{
    Context, EmptySubscription, ErrorExtensions, FieldError, FieldResult, Object, Schema,
};
use chrono::Utc;
use futures_util::lock::Mutex;
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use std::{env, sync::Arc};

pub type DataSchema = Schema<Query, Mutation, EmptySubscription>;
use slab::Slab;

use crate::middlewares::{Claims, TokenGuard, EXP};
pub struct Query;

pub type Storage = Arc<Mutex<Slab<String>>>;

#[Object]
impl Query {
    #[graphql(guard = "TokenGuard")]
    async fn get(&self, ctx: &Context<'_>) -> FieldResult<String> {
        let data = ctx.data_unchecked::<Storage>().lock().await;
        if data.is_empty() {
            Ok("".to_string())
        } else {
            Ok(data[0].clone())
        }
    }
    async fn login(
        &self,
        _ctx: &Context<'_>,
        username: String,
        password: String,
    ) -> FieldResult<TokenResponse> {
        if username == env::var("PORTAL_USERNAME").unwrap()
            && password == env::var("PORTAL_PASSWORD").unwrap()
        {
            let now = Utc::now().timestamp();
            let claims = Claims {
                user: username,
                exp: now + EXP,
                iat: now,
            };
            let key = env::var("PORTAL_JWT_KEY").unwrap();
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
                }
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
    #[graphql(guard = "TokenGuard")]
    async fn set(&self, ctx: &Context<'_>, data: String) -> FieldResult<bool> {
        if data.is_empty() {
            return Err(FieldError::from("请输入有效内容!"))
                .map_err(|err| err.extend_with(|_, e| e.set("code", "INVALID_INPUT")));
        }
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

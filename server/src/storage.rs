use async_graphql::{Context, EmptySubscription, FieldResult, Object, Schema};
use futures_util::lock::Mutex;
use std::sync::Arc;

pub type DataSchema = Schema<Query, Mutation, EmptySubscription>;
use slab::Slab;
pub struct Query;

pub type Storage = Arc<Mutex<Slab<String>>>;

#[Object]
impl Query {
    async fn get(&self, ctx: &Context<'_>) -> String {
        let data = ctx.data_unchecked::<Storage>().lock().await;
        if data.is_empty() {
            "".to_string()
        } else {
            data[0].clone()
        }
    }
}

pub struct Mutation;

#[Object]
impl Mutation {
    async fn set(&self, ctx: &Context<'_>, data: String) -> FieldResult<bool> {
        let mut storage = ctx.data_unchecked::<Storage>().lock().await;
        storage.clear();
        storage.insert(data);
        Ok(true)
    }
}

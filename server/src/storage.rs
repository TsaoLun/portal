use async_graphql::{Object, Context, Subscription, Schema, FieldResult};
use futures_util::{lock::Mutex, Stream};
use std::sync::Arc;
use std::time::Duration;

pub type DataSchema = Schema<Query, Mutation, Subscription>;
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

#[derive(Clone)]
pub struct SetResponse {
    ok: bool
}

#[Object]
impl SetResponse {
    async fn ok(&self) -> bool {
        self.ok
    }
}

pub struct Mutation;

#[Object]
impl Mutation {
    async fn set(&self, ctx: &Context<'_>, data: String) -> SetResponse {
        println!("{}", data);
        let mut storage = ctx.data_unchecked::<Storage>().lock().await;
        storage.clear();
        storage.insert(data);
        SetResponse{ ok: true }
    }
}

pub struct Subscription;

#[Subscription]
impl Subscription {
    async fn interval(&self, #[graphql(default = 1)] n: i32) -> impl Stream<Item = i32> {
        let mut value = 0;
        async_stream::stream! {
            loop {
                futures_timer::Delay::new(Duration::from_secs(1)).await;
                value += n;
                yield value;
            }
        }
    }
}
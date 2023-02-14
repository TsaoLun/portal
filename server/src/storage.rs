use async_graphql::{Object, Context, Subscription, Schema};
use futures_util::{lock::Mutex, Stream};
use std::sync::Arc;
use std::time::Duration;

pub type DataSchema = Schema<QueryRoot, MutationRoot, SubscriptionRoot>;
use slab::Slab;
pub struct QueryRoot;

pub type Storage = Arc<Mutex<Slab<String>>>;

#[Object]
impl QueryRoot {
    async fn get(&self, ctx: &Context<'_>) -> String {
        let data = ctx.data_unchecked::<Storage>().lock().await;
        if data.is_empty() {
            "".to_string()
        } else {
            data[0].clone()
        }
    }
}

pub struct MutationRoot;

#[Object]
impl MutationRoot {
    async fn set(&self, ctx: &Context<'_>, data: String) -> bool {
        let mut storage = ctx.data_unchecked::<Storage>().lock().await;
        storage.clear();
        storage.insert(data);
        true
    }
}

pub struct SubscriptionRoot;

#[Subscription]
impl SubscriptionRoot {
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
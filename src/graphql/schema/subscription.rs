extern crate derive_more;

use async_graphql::{
    http::{playground_source, GraphQLPlaygroundConfig},
    Context, Data, Object, Schema, Subscription,
};

use async_graphql::EmptyMutation;
use std::sync::{Arc, Mutex, MutexGuard, PoisonError};
use std::time::Duration;

use derive_more::{Add, Display, From, Into, Unwrap};

use graphql_actix_web_lib;
use graphql_actix_web_lib::{GraphQLRequest, GraphQLResponse, GraphQLSubscription};

use deadpool::unmanaged::Pool;
use futures_util::Stream;
use serde::Deserialize;

use crate::db::RedisPool;
use crate::graphql::context::{ContextData, GqlContext};

pub struct RootSubscription;

#[Subscription]
impl RootSubscription {
    async fn values(
        &self,
        ctx: &Context<'_>,
    ) -> actix_web::Result<impl Stream<Item = i32>, async_graphql::Error> {
        {
            let gql_context = ctx.data::<GqlContext>()?;
            let session_data = gql_context.lock().unwrap();
            println!("<><><>: {:?}", session_data);
        }
        // if ctx.data::<Token>()?.0 != "123456" {
        //     return Err("Forbidden".into());
        // }
        Ok(futures_util::stream::once(async move { 10 }))
    }

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

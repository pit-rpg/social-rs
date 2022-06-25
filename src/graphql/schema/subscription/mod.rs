extern crate derive_more;
use crate::controllers::{ChatMessageChange, ControllerChat};
use crate::error::GQLResult;
use crate::graphql::utils::get_user_id;
use async_graphql::{Context, Subscription};
use futures_util::{Stream};
use mongodb::Database;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::Duration;

#[derive(Serialize, Debug, Default, Clone, Deserialize)]
pub struct RootSubscription;

#[Subscription]
impl RootSubscription {
    async fn watch_messages(
        &self,
        ctx: &Context<'_>,
        chat: String,
    ) -> impl Stream<Item = GQLResult<ChatMessageChange>> {
        let id = get_user_id(ctx);
        let db = ctx.data::<Arc<Database>>().unwrap();

        ControllerChat::watch_messages(db, id, chat).await
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

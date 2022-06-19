use async_graphql::{Object};
use serde::{Deserialize, Serialize};

mod session;
mod chat;

use session::QuerySession;
use chat::QueryChat;

#[derive(Serialize, Debug, Default, Clone, Deserialize)]

pub struct RootQuery;
#[Object]
impl RootQuery {
    async fn session(&self) -> QuerySession {
        QuerySession::default()
    }
    async fn chat(&self) -> QueryChat {
        QueryChat::default()
    }
}

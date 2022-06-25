use serde::{Deserialize, Serialize};
use async_graphql::{Enum};

mod user;
mod chat;

pub use user::*;
pub use chat::*;

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Enum, Copy)]
pub enum SubscriptionChange {
    New,
    Delete,
    Update,
}
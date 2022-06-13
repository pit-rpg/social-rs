use crate::db::{validations::validate_trimmed, utils::{CollectionUtils, date_now}};
use async_graphql::{Enum, InputObject, SimpleObject};
use mongodb::{bson::doc, bson::Uuid, options::IndexOptions, Collection, Database, IndexModel, bson::Timestamp};
use serde::{Deserialize, Serialize};
use std::result::Result;
use validator::Validate;
use std::time::{Duration, Instant, UNIX_EPOCH, SystemTime};
use std::default::{Default};

pub struct Chat;

impl Chat {
    pub async fn create() {
        unimplemented!()
    }

    pub async fn get_chats() {
        unimplemented!()
    }
}

#[derive(Debug, Serialize, Deserialize, Validate, Default, Clone)]
pub struct DBChat {
    #[serde(rename = "_id")]
    pub id: Uuid,

    pub users: Vec<Uuid>,

    pub private: bool,

    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Validate, Clone)]
pub struct DBChatMessage {
    #[serde(rename = "_id")]
    pub id: Uuid,

    pub chat_id: Uuid,

    pub user_id: Uuid,

    pub edit: Option<u64>,

    pub time: u64,

    pub message: Message,
}

impl DBChatMessage {
    pub fn new(chat_id: Uuid, user_id: Uuid, message: Message) -> Self {
        DBChatMessage {
            id: Uuid::default(),
            edit: None,
            time: date_now(),
            user_id,
            chat_id,
            message,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Message {
    Text(String),
    Emoji(char),
}



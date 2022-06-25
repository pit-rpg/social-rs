use crate::db::utils::CollectionUtils;
use async_graphql::Enum;
use mongodb::{
    bson::doc, bson::oid::ObjectId, options::IndexOptions, Collection, Database, IndexModel,
};
use serde::{Deserialize, Serialize};
use validator::Validate;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Validate, Clone)]
pub struct DBChat {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_id")]
    pub id: Option<ObjectId>,
    pub owner: Option<ObjectId>,
    pub users: Vec<ObjectId>,
    pub chat_type: ChatType,
    pub name: Option<String>,
    pub last_msg: Option<ObjectId>,
    pub user_last_msg: HashMap<String, ObjectId>,
}

impl DBChat {
    pub fn new(
        users: &[ObjectId],
        chat_type: ChatType,
        name: Option<String>,
        owner: Option<ObjectId>,
    ) -> Self {
        Self {
            id: None,
            users: users.into(),
            chat_type,
            owner,
            name,
            last_msg: None,
            user_last_msg: HashMap::new(),
        }
    }

    pub async fn create_indexes(db: &Database) {
        let options = IndexOptions::builder().build();

        let model1 = IndexModel::builder()
            .keys(doc! {"users": 1u32})
            .options(options.clone())
            .build();

        DBChat::to_collection(&db)
            .create_indexes(vec![model1], None)
            .await
            .expect("error creating index!");
    }
}

#[derive(Debug, Serialize, Deserialize, Validate, Clone)]
pub struct DBChatMessage {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_id")]
    pub id: Option<ObjectId>,
    pub chat: ObjectId,
    pub user: ObjectId,
    pub edit: Option<u64>,
    pub message: String,
}

impl DBChatMessage {
    pub fn new(chat: ObjectId, user: ObjectId, message: String) -> Self {
        DBChatMessage {
            id: None,
            edit: None,
            user,
            chat,
            message,
        }
    }

    pub async fn create_indexes(db: &Database) {
        let options = IndexOptions::builder().build();

        let model1 = IndexModel::builder()
            .keys(doc! {"time": 1u32})
            .options(options.clone())
            .build();
        let model2 = IndexModel::builder()
            .keys(doc! {"chat_id": 1u32})
            .options(options.clone())
            .build();

        DBChatMessage::to_collection(&db)
            .create_indexes(vec![model1, model2], None)
            .await
            .expect("error creating index!");
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Enum, Copy, PartialEq, Eq)]
pub enum ChatType {
    UserPrivate,
    Private,
    Group,
}

impl CollectionUtils<DBChat> for DBChat {
    fn to_collection(db: &Database) -> Collection<DBChat> {
        let name = Self::get_collection_name();
        db.collection::<DBChat>(name)
    }

    fn get_collection_name() -> &'static str {
        "Chat"
    }
}

impl CollectionUtils<DBChatMessage> for DBChatMessage {
    fn to_collection(db: &Database) -> Collection<DBChatMessage> {
        let name = Self::get_collection_name();
        db.collection::<DBChatMessage>(name)
    }

    fn get_collection_name() -> &'static str {
        "ChatMessage"
    }
}

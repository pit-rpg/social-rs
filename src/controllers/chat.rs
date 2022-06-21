use crate::db::{utils::{CollectionUtils, map_id_to_string}, ChatType, DBChat, DBChatMessage};
use crate::graphql::utils::GQLResult;
use async_graphql::connection::{Connection, Edge, EmptyFields};
use async_graphql::{SimpleObject};
use futures::stream::TryStreamExt;
use mongodb::bson::to_bson;
use mongodb::bson::Document;
use mongodb::{bson::doc, bson::oid::ObjectId, options::FindOptions, Database};
use serde::{Deserialize, Serialize};

lazy_static! {
    static ref ARGON_2_CONF: argon2::Config<'static> = argon2::Config::default();
}

pub struct Chat;

impl Chat {
    pub async fn create_user_private(
        db: &Database,
        user: &ObjectId,
    ) -> GQLResult<OutputChat> {
        let chat = DBChat::new(&[*user], ChatType::UserPrivate, None, Some(user.clone()));
        DBChat::to_collection(db)
            .insert_one(&chat, None)
            .await
            .or(Err("can't create chat"))?;

        Ok(chat.into())
    }

    pub async fn create_private(
        db: &Database,
        session_user: &ObjectId,
        user: &ObjectId,
    ) -> GQLResult<OutputChat> {
        let mut chat = DBChat::new(
            &[*session_user, *user],
            ChatType::Private,
            None,
            Some(user.clone()),
        );

        chat.id = DBChat::to_collection(db)
            .insert_one(&chat, None)
            .await
            .or(Err("can't create chat"))?
            .inserted_id
            .as_object_id();

        Ok(chat.into())
    }

    pub async fn get_chats(
        db: &Database,
        user: &ObjectId,
        after: Option<ObjectId>,
        before: Option<ObjectId>,
        first: Option<i64>,
    ) -> GQLResult<Connection<String, OutputChat, EmptyFields, EmptyFields>> {
        if before.is_none() && after.is_some() {
            Err("The \"before\" and \"after\" parameters cannot exist at the same time")?;
        }

        let first = first.or(Some(50)).unwrap().min(500).max(0);
        let find_options;
        let query;

        if after.is_some() {
            find_options = FindOptions::builder()
                .limit(first)
                .sort(doc! { "_id": 1i32});
            query = doc! {"users": user, "_id": {"$gt": after.unwrap()}};
        } else if before.is_some() {
            find_options = FindOptions::builder()
                .limit(first)
                .sort(doc! { "_id": -1i32});
            query = doc! {"users": user, "_id": {"$lt": before.unwrap()}};
        } else {
            find_options = FindOptions::builder()
                .limit(first)
                .sort(doc! { "_id": 1i32});
            query = doc! {"users": user};
        }

        let chats = DBChat::to_collection(db)
            .find(query, find_options.build())
            .await
            .or(Err("cent make query for chats"))?
            .try_collect::<Vec<DBChat>>()
            .await
            .or(Err("cent get chats"))?;

        let has_next_page = first == chats.len() as i64;
        let has_previous_page = after.is_some() || before.is_some();
        let mut connection = Connection::new(has_previous_page, has_next_page);

        connection.edges.extend(chats.into_iter().map(|chat| {
            Edge::with_additional_fields(chat.id.unwrap().to_string(), chat.into(), EmptyFields)
        }));

        Ok(connection)
    }

    pub async fn send_message(
        db: &Database,
        owner: ObjectId,
        chat: ObjectId,
        message: String,
    ) -> GQLResult<OutputChatMessage, &'static str> {
        let query_chat = doc! {"_id": chat, "users": owner};
        let mut message = DBChatMessage::new(chat, owner, message);

        DBChat::to_collection(db)
            .find_one(query_chat, None)
            .await
            .or(Err("You don't have access"))?;

        let id = DBChatMessage::to_collection(db)
            .insert_one(&message, None)
            .await
            .or(Err("cent send message"))?
            .inserted_id
            .as_object_id()
            .expect("cen't serialize ObjectId");

        message.id = Some(id);

        Ok(message.into())
    }

    pub async fn get_messages(
        db: &Database,
        user: &ObjectId,
        chat: ObjectId,
        after: Option<ObjectId>,
        before: Option<ObjectId>,
        first: Option<i64>,
    ) -> GQLResult<Connection<String, OutputChatMessage, EmptyFields, EmptyFields>> {
        if before.is_none() && after.is_some() {
            Err("The \"before\" and \"after\" parameters cannot exist at the same time")?;
        }

        let first = first.or(Some(50)).unwrap().min(500).max(0);
        let find_options;
        let query;

        DBChat::to_collection(db)
            .find_one(doc! {"_id": chat, "users": user}, None)
            .await
            .or(Err("You don't have access"))?;

        if after.is_some() {
            find_options = FindOptions::builder()
                .limit(first)
                .sort(doc! { "_id": 1i32});
            query = doc! {"chat": chat, "_id": {"$gt": after.unwrap()}};
        } else if before.is_some() {
            find_options = FindOptions::builder()
                .limit(first)
                .sort(doc! { "_id": -1i32});
            query = doc! {"chat": chat, "_id": {"$lt": before.unwrap()}};
        } else {
            find_options = FindOptions::builder()
                .limit(first)
                .sort(doc! { "_id": 1i32});
            query = doc! {"chat": chat};
        }

        let messages = DBChatMessage::to_collection(db)
            .find(query, find_options.build())
            .await
            .or(Err("cent make query for messages"))?
            .try_collect::<Vec<DBChatMessage>>()
            .await
            .or(Err("cent get messages"))?;

        let has_next_page = first == messages.len() as i64;
        let has_previous_page = after.is_some() || before.is_some();
        let mut connection = Connection::new(has_previous_page, has_next_page);

        connection.edges.extend(messages.into_iter().map(|msg| {
            Edge::with_additional_fields(msg.id.unwrap().to_string(), msg.into(), EmptyFields)
        }));

        Ok(connection)
    }

    pub async fn remove_messages(
        db: &Database,
        owner: ObjectId,
        chat: ObjectId,
        messages: Vec<ObjectId>,
    ) -> GQLResult<bool> {
        let query_chat = doc! {"_id": chat, "users": owner};

        DBChat::to_collection(db)
            .find_one(query_chat, None)
            .await
            .or(Err("You don't have access"))?;

        DBChatMessage::to_collection(db)
            .delete_many(doc! {"_id": {"$in":messages}}, None)
            .await
            .or(Err("cent send message"))?;

        Ok(true)
    }

    pub async fn remove_chat(
        db: &Database,
        owner: ObjectId,
        chat_id: ObjectId,
    ) -> GQLResult<bool> {
        let query_chat = doc! {"_id": chat_id, "owner": owner};
        let variant = to_bson(&ChatType::Private).unwrap();

        let mut chat = DBChat::to_collection(db)
            .find_one(query_chat, None)
            .await
            .or(Err("You don't have access"))?;

        if chat.is_none() {
            let mut query_chat = Document::new();
            query_chat.insert("_id", chat_id);
            query_chat.insert("owner", owner);
            query_chat.insert("chat_type", variant);

            chat = DBChat::to_collection(db)
                .find_one(query_chat, None)
                .await
                .or(Err("You don't have access"))?;
        }

        chat.expect("Cen't delete chat");

        DBChatMessage::to_collection(db)
            .delete_many(doc! {"chat": chat_id}, None)
            .await
            .or(Err("Can't delete messages"))?;

        DBChat::to_collection(db)
            .delete_one(doc! {"_id": chat_id}, None)
            .await
            .or(Err("Can't delete chat"))?;

        Ok(true)
    }
}

#[derive(SimpleObject, Debug, Serialize, Deserialize, Clone)]
pub struct OutputChat {
    pub id: Option<String>,
    pub owner: Option<String>,
    pub users: Option<Vec<String>>,
    pub chat_type: Option<ChatType>,
    pub name: Option<String>,
}

impl From<&DBChat> for OutputChat {
    fn from(item: &DBChat) -> OutputChat {
        OutputChat {
            id: map_id_to_string(&item.id),
            chat_type: Some(item.chat_type),
            name: item.name.clone(),
            owner: item.owner.map(|i| i.to_string()),
            users: Some(item.users.iter().map(|i| i.to_string()).collect()),
        }
    }
}

impl From<DBChat> for OutputChat {
    fn from(item: DBChat) -> OutputChat {
        (&item).into()
    }
}

#[derive(SimpleObject, Debug, Serialize, Deserialize, Clone)]
pub struct OutputChatMessage {
    pub id: Option<String>,
    pub chat: Option<String>,
    pub user: Option<String>,
    pub edit: Option<u64>,
    pub message: Option<String>,
}

impl From<&DBChatMessage> for OutputChatMessage {
    fn from(item: &DBChatMessage) -> OutputChatMessage {
        OutputChatMessage {
            id: map_id_to_string(&item.id),
            chat: Some(item.chat.to_string()),
            user: Some(item.user.to_string()),
            edit: item.edit,
            message: Some(item.message.clone()),
        }
    }
}

impl From<DBChatMessage> for OutputChatMessage {
    fn from(item: DBChatMessage) -> OutputChatMessage {
        (&item).into()
    }
}
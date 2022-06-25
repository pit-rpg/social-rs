use crate::{db::{
    ChatType, DBChat, DBChatMessage,
    {map_id_hash_map, map_id_to_string, map_string_to_id, CollectionUtils},
}, error::Error};
use crate::error::GQLResult;
use super::SubscriptionChange;
use crate::graphql::simple_broker::SimpleBroker;
use async_graphql::connection::{query, Connection, Edge, EmptyFields};
use async_graphql::{Object, SimpleObject};
use futures::stream::TryStreamExt;
use mongodb::bson::to_bson;
use mongodb::bson::Document;
use mongodb::{bson::doc, bson::oid::ObjectId, options::FindOptions, Database};
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use futures_util::{Stream, StreamExt};

lazy_static! {
    static ref ARGON_2_CONF: argon2::Config<'static> = argon2::Config::default();
}

pub struct ControllerChat;

impl ControllerChat {
    pub async fn create_user_private(db: &Database, user: &ObjectId) -> GQLResult<DBChat> {
        let chat = DBChat::new(&[*user], ChatType::UserPrivate, None, Some(user.clone()));
        DBChat::to_collection(db).insert_one(&chat, None).await?;

        Ok(chat)
    }

    pub async fn create_private(
        db: &Database,
        session_user: &ObjectId,
        user: &ObjectId,
    ) -> GQLResult<DBChat> {
        let mut chat = DBChat::new(
            &[*session_user, *user],
            ChatType::Private,
            None,
            Some(user.clone()),
        );

        chat.id = DBChat::to_collection(db)
            .insert_one(&chat, None)
            .await?
            .inserted_id
            .as_object_id();

        Ok(chat)
    }

    pub async fn get_chats(
        db: &Database,
        user: &ObjectId,
        after: Option<String>,
        before: Option<String>,
        first: Option<i32>,
        last: Option<i32>,
    ) -> GQLResult<Connection<String, DBChat, EmptyFields, EmptyFields>> {
        if first.is_some() && last.is_some() {
            Err("do not use 'first' && 'last' at the same time")?
        }

        let res = query(
            after,
            before,
            first,
            last,
            |after, before, first, last| async move {
                let after = map_string_to_id(&after);
                let before = map_string_to_id(&before);
                let limit = first.or(last).or(Some(50)).unwrap().min(500).max(0) as i64;

                let (find_options, query) = match (after, before) {
                    (_, Some(before)) => {
                        let find_options = FindOptions::builder()
                            .limit(limit)
                            .sort(doc! { "last_msg": -1i32, "_id": -1i32})
                            .build();
                        let query = doc! {"users": user, "_id": {"$lt": before}};
                        (find_options, query)
                    }
                    (Some(after), _) => {
                        let find_options = FindOptions::builder()
                            .limit(limit)
                            .sort(doc! { "last_msg": 1i32, "_id": 1i32})
                            .build();
                        let query = doc! {"users": user, "_id": {"$gt": after}};
                        (find_options, query)
                    }
                    _ => {
                        let find_options = FindOptions::builder()
                            .limit(limit)
                            .sort(doc! { "last_msg": 1i32, "_id": 1i32})
                            .build();
                        let query = doc! {"users": user};
                        (find_options, query)
                    }
                };

                let chats = DBChat::to_collection(db)
                    .find(query, find_options)
                    .await?
                    .try_collect::<Vec<DBChat>>()
                    .await?;

                let hes_all = limit as usize == chats.len();
                let has_next_page = after.is_some() && hes_all || after.is_some();
                let has_previous_page = after.is_some() || before.is_some() && hes_all;

                let mut connection =
                    Connection::<String, DBChat>::new(has_previous_page, has_next_page);

                let edges = chats
                    .into_iter()
                    .map(|n| {
                        Ok(Edge::with_additional_fields(
                            map_id_to_string(&n.id).ok_or("cen't get Id")?,
                            n,
                            EmptyFields,
                        ))
                    })
                    .collect::<GQLResult<Vec<Edge<String, DBChat, _>>>>()?;

                connection.edges.extend(edges);

                Ok::<_, async_graphql::Error>(connection)
            },
        )
        .await?;

        Ok(res)
    }

    pub async fn send_message(
        db: &Database,
        owner: ObjectId,
        chat: ObjectId,
        message: String,
    ) -> GQLResult<DBChatMessage> {
        let query_chat_check = doc! {"_id": chat, "users": owner};
        let query_chat = doc! {"_id": chat};
        let mut message = DBChatMessage::new(chat, owner, message);

        DBChat::to_collection(db)
            .find_one(query_chat_check, None)
            .await
            .or(Err("You don't have access"))?;

        let id = DBChatMessage::to_collection(db)
            .insert_one(&message, None)
            .await?
            .inserted_id
            .as_object_id()
            .ok_or("cen't serialize ObjectId")?;

        let update = doc! {"$set": doc!{
            format!("user_last_msg.{}", owner.to_string()): id,
            "last_msg": id,
        }};

        println!("====> {}", update);

        DBChat::to_collection(db)
            .update_one(query_chat, update, None)
            .await?;

        message.id = Some(id);

        SimpleBroker::publish(ChatMessageChange {
            change: SubscriptionChange::New,
            message: message.clone()
        });

        Ok(message)
    }

    pub async fn get_messages(
        db: &Database,
        user: &ObjectId,
        chat: ObjectId,
        after: Option<String>,
        before: Option<String>,
        first: Option<i32>,
        last: Option<i32>,
    ) -> GQLResult<Connection<String, DBChatMessage, EmptyFields, EmptyFields>> {
        if first.is_some() && last.is_some() {
            Err("do not use 'first' && 'last' at the same time")?
        }

        DBChat::to_collection(db)
            .find_one(doc! {"_id": chat, "users": user}, None)
            .await
            .or(Err("You don't have access"))?;

        let res = query(
            after,
            before,
            first,
            last,
            |after, before, first, last| async move {
                let after = map_string_to_id(&after);
                let before = map_string_to_id(&before);
                let limit = first.or(last).or(Some(50)).unwrap().min(500).max(0) as i64;

                let (find_options, query) = match (after, before) {
                    (_, Some(before)) => {
                        let find_options = FindOptions::builder()
                            .limit(limit)
                            .sort(doc! { "_id": -1i32})
                            .build();
                        let query = doc! {"chat": chat, "_id": {"$lt": before}};
                        (find_options, query)
                    }
                    (Some(after), _) => {
                        let find_options = FindOptions::builder()
                            .limit(limit)
                            .sort(doc! { "_id": 1i32})
                            .build();
                        let query = doc! {"chat": chat, "_id": {"$gt": after}};
                        (find_options, query)
                    }
                    _ => {
                        let find_options = FindOptions::builder()
                            .limit(limit)
                            .sort(doc! { "_id": 1i32})
                            .build();
                        let query = doc! {"chat": chat};
                        (find_options, query)
                    }
                };

                let messages = DBChatMessage::to_collection(db)
                    .find(query, find_options)
                    .await
                    .or(Err("cent make query for messages"))?
                    .try_collect::<Vec<DBChatMessage>>()
                    .await
                    .or(Err("cent get messages"))?;

                let hes_all = limit as usize == messages.len();
                let has_next_page = after.is_some() && hes_all || after.is_some();
                let has_previous_page = after.is_some() || before.is_some() && hes_all;

                let mut connection =
                    Connection::<String, DBChatMessage>::new(has_previous_page, has_next_page);

                println!("MESSAGES: {:?}", messages);

                let edges = messages
                    .into_iter()
                    .map(|n| {
                        Ok(Edge::with_additional_fields(
                            map_id_to_string(&n.id).ok_or("cen't get Id")?,
                            n,
                            EmptyFields,
                        ))
                    })
                    .collect::<GQLResult<Vec<Edge<String, DBChatMessage, _>>>>()?;

                connection.edges.extend(edges);

                Ok::<_, async_graphql::Error>(connection)
            },
        )
        .await?;

        Ok(res)
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
            .delete_many(doc! {"_id": {"$in": &messages}}, None)
            .await
            .or(Err("cent send message"))?;

        messages
            .into_iter()
            .map(|id| {
                let mut msg = ChatMessageChange{
                    change: SubscriptionChange::Delete,
                    message: DBChatMessage::new(chat, owner, "".to_string())
                };

                msg.message.id = Some(id);
                msg
            })
            .for_each(|i| SimpleBroker::publish(i));

        Ok(true)
    }

    pub async fn remove_chat(db: &Database, owner: ObjectId, chat_id: ObjectId) -> GQLResult<bool> {
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

    pub async fn watch_messages(
        db: &Database,
        owner: GQLResult<ObjectId>,
        chat: String,
    ) -> impl Stream<Item = GQLResult<ChatMessageChange>> {
        let owner = owner.ok();
        let chat = ObjectId::parse_str(chat).ok();
        let mut res = None;

        if owner.is_some() && chat.is_some() {
            let query_chat = doc! {"_id": chat.unwrap(), "users": owner.unwrap()};

            res = DBChat::to_collection(db)
                .find_one(query_chat, None)
                .await
                .ok()
                .flatten();
        }

        SimpleBroker::<ChatMessageChange>::subscribe()
            .map(move |event| {
                if res.is_none() {
                    return Err(Error::new_str("You don't have access"));
                }

                if event.message.chat != chat.unwrap() {
                    return Ok(None);
                }

                Ok(Some(event))
            })
            .filter(move |event| {
                let res = event.is_err() || event.as_ref().unwrap().is_some();
                async move {res}
            })
            .map(move |event| event.map(|i| i.unwrap()))
    }
}


#[Object(name="Chat")]
impl DBChat {
    pub async fn id(&self) -> Option<String> {
        map_id_to_string(&self.id)
    }
    pub async fn owner(&self) -> Option<String> {
        map_id_to_string(&self.owner)
    }
    pub async fn users(&self) -> Option<Vec<String>> {
        Some(self.users.iter().map(|i| i.to_string()).collect())
    }
    pub async fn chat_type(&self) -> Option<ChatType> {
        Some(self.chat_type)
    }
    pub async fn name(&self) -> Option<String> {
        self.name.clone()
    }
    pub async fn last_msg(&self) -> Option<String> {
        map_id_to_string(&self.last_msg)
    }
    pub async fn user_last_msg(&self) -> Option<HashMap<String, String>> {
        Some(map_id_hash_map(&self.user_last_msg))
    }
}

#[Object(name="ChatMessage")]
impl DBChatMessage {
    pub async fn id(&self) -> Option<String>{
        map_id_to_string(&self.id)
    }
    pub async fn chat(&self) -> Option<String>{
        Some(self.chat.to_string())
    }
    pub async fn user(&self) -> Option<String>{
        Some(self.user.to_string())
    }
    pub async fn edit(&self) -> Option<u64>{
        self.edit.clone()
    }
    pub async fn message(&self) -> Option<String>{
        Some(self.message.clone())
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, SimpleObject)]
pub struct ChatMessageChange {
    change: SubscriptionChange,
    message: DBChatMessage
}
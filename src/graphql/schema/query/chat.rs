use crate::controllers::{ControllerChat};
use crate::db::{DBChat, DBChatMessage};
use crate::graphql::utils::{get_user_id};
use crate::error::GQLResult;
use async_graphql::connection::{Connection, EmptyFields};
use async_graphql::{Context, Object};
use mongodb::bson::oid::ObjectId;
use mongodb::Database;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Serialize, Debug, Default, Clone, Deserialize)]
pub struct QueryChat;

#[Object]
impl QueryChat {
    async fn create_user_private<'a>(&self, ctx: &'a Context<'_>) -> GQLResult<Option<DBChat>> {
        let id = get_user_id(ctx)?;
        let db = ctx.data::<Arc<Database>>().unwrap();
        let chat = ControllerChat::create_user_private(db, &id).await?;

        Ok(Some(chat))
    }

    async fn create_private<'a>(
        &self,
        ctx: &'a Context<'_>,
        user_id: String,
    ) -> GQLResult<Option<DBChat>> {
        let id = get_user_id(ctx)?;
        let user_id: ObjectId = ObjectId::parse_str(&user_id)?;
        let db = ctx.data::<Arc<Database>>().unwrap();
        let chat = ControllerChat::create_private(db, &id, &user_id).await?;

        Ok(Some(chat))
    }

    async fn get_chats<'a>(
        &self,
        ctx: &'a Context<'_>,
        after: Option<String>,
        before: Option<String>,
        first: Option<i32>,
        last: Option<i32>,
    ) -> GQLResult<Connection<String, DBChat, EmptyFields, EmptyFields>> {
        let id = get_user_id(ctx)?;
        let db = ctx.data::<Arc<Database>>().unwrap();

        ControllerChat::get_chats(db, &id, after, before, last, first).await
    }

    async fn get_messages<'a>(
        &self,
        ctx: &'a Context<'_>,
        chat: String,
        after: Option<String>,
        before: Option<String>,
        first: Option<i32>,
        last: Option<i32>,
    ) -> GQLResult<Connection<String, DBChatMessage, EmptyFields, EmptyFields>> {
        let chat = ObjectId::parse_str(chat)?;
        let id = get_user_id(ctx)?;
        let db = ctx.data::<Arc<Database>>().unwrap();

        ControllerChat::get_messages(db, &id, chat, after, before, last, first).await
    }

    async fn send_message<'a>(
        &self,
        ctx: &'a Context<'_>,
        chat: String,
        message: String,
    ) -> GQLResult<Option<DBChatMessage>> {
        let chat = ObjectId::parse_str(chat)?;
        let id = get_user_id(ctx)?;

        let db = ctx.data::<Arc<Database>>().unwrap();
        let message = ControllerChat::send_message(db, id, chat, message).await?;

        Ok(Some(message))
    }

    async fn remove_messages<'a>(
        &self,
        ctx: &'a Context<'_>,
        chat: String,
        messages: Vec<String>,
    ) -> GQLResult<Option<bool>> {
        let chat = ObjectId::parse_str(chat)?;
        let messages = messages
            .into_iter()
            .map(|id| ObjectId::parse_str(id).ok())
            .flatten()
            .collect::<Vec<ObjectId>>();

        let id = get_user_id(ctx)?;

        let db = ctx.data::<Arc<Database>>().unwrap();
        let res = ControllerChat::remove_messages(db, id, chat, messages).await?;

        Ok(Some(res))
    }

    async fn remove_chat<'a>(&self, ctx: &'a Context<'_>, chat: String) -> GQLResult<Option<bool>> {
        let chat = ObjectId::parse_str(chat)?;
        let id = get_user_id(ctx)?;

        let db = ctx.data::<Arc<Database>>().unwrap();
        let res = ControllerChat::remove_chat(db, id, chat).await?;

        Ok(Some(res))
    }
}

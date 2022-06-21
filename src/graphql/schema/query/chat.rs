use crate::controllers::{Chat, OutputChat, OutputChatMessage};
use crate::graphql::utils::{get_user_id, GQLResult};
use crate::db::utils::{map_string_to_id};
use async_graphql::connection::Connection;
use async_graphql::{Context, Object, Result};
use mongodb::bson::oid::ObjectId;
use mongodb::Database;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Serialize, Debug, Default, Clone, Deserialize)]
pub struct QueryChat;

#[Object]
impl QueryChat {
    async fn create_user_private<'a>(&self, ctx: &'a Context<'_>) -> GQLResult<Option<OutputChat>> {
        let id = get_user_id(ctx)?;
        let db = ctx.data::<Arc<Database>>().unwrap();
        let chat = Chat::create_user_private(db, &id).await?;

        Ok(Some(chat))
    }

    async fn create_private<'a>(
        &self,
        ctx: &'a Context<'_>,
        user_id: String,
    ) -> GQLResult<Option<OutputChat>> {
        let id = get_user_id(ctx)?;
        let user_id: ObjectId = ObjectId::parse_str(&user_id)?;
        let db = ctx.data::<Arc<Database>>().unwrap();
        let chat = Chat::create_private(db, &id, &user_id).await?;

        Ok(Some(chat))
    }

    async fn get_chats<'a>(
        &self,
        ctx: &'a Context<'_>,
        after: Option<String>,
        before: Option<String>,
        first: Option<i64>,
    ) -> GQLResult<Connection<String, OutputChat>> {
        let after = map_string_to_id(&after);
        let before = map_string_to_id(&before);
        let id = get_user_id(ctx)?;

        let db = ctx.data::<Arc<Database>>().unwrap();
        let chats = Chat::get_chats(db, &id, after, before, first).await?;

        Ok(chats)
    }

    async fn get_messages<'a>(
        &self,
        ctx: &'a Context<'_>,
        chat: String,
        after: Option<String>,
        before: Option<String>,
        first: Option<i64>,
    ) -> GQLResult<Connection<String, OutputChatMessage>> {
        let after = map_string_to_id(&after);
        let before = map_string_to_id(&before);
        let chat = ObjectId::parse_str(chat)?;
        let id = get_user_id(ctx)?;

        let db = ctx.data::<Arc<Database>>().unwrap();
        let messages = Chat::get_messages(db, &id, chat, after, before, first).await?;

        Ok(messages)
    }


    async fn send_message<'a>(
        &self,
        ctx: &'a Context<'_>,
        chat: String,
        message: String,
    ) -> GQLResult<Option<OutputChatMessage>> {
        let chat = ObjectId::parse_str(chat)?;
        let id = get_user_id(ctx)?;

        let db = ctx.data::<Arc<Database>>().unwrap();
        let message = Chat::send_message(db, id, chat, message).await?;

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
        let res = Chat::remove_messages(db, id, chat, messages).await?;

        Ok(Some(res))
    }

    async fn remove_chat<'a>(
        &self,
        ctx: &'a Context<'_>,
        chat: String,
    ) -> GQLResult<Option<bool>> {
        let chat = ObjectId::parse_str(chat)?;
        let id = get_user_id(ctx)?;

        let db = ctx.data::<Arc<Database>>().unwrap();
        let res = Chat::remove_chat(db, id, chat).await?;

        Ok(Some(res))
    }
}

use crate::db::models::{InputUserLogin, OutputUser, User};
use crate::graphql::context::GqlContext;
use async_graphql::{Context, Object, Result};
use mongodb::{bson::Uuid, Database};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Serialize, Debug, Default, Clone, Deserialize)]
pub struct MutationSession;

#[Object]
impl MutationSession {
    async fn register<'a>(&self, ctx: &'a Context<'_>, data: InputUserLogin) -> Result<OutputUser> {
        let user = {
            let db = ctx.data::<Arc<Database>>().unwrap();
            User::register(db, data).await?
        };

        {
            let gql_session = ctx.data::<GqlContext>().unwrap();
            let mut session_data = gql_session.lock().unwrap();
            session_data.user_id = Some(user.id.to_string());
        }

        Ok(user.into())
    }

    async fn log_in<'a>(&self, ctx: &'a Context<'_>, data: InputUserLogin) -> Result<OutputUser> {
        let user = {
            let db = ctx.data::<Arc<Database>>().unwrap();
            User::log_in(db, data).await?
        };

        {
            let gql_session = ctx.data::<GqlContext>().unwrap();
            let mut session_data = gql_session.lock().unwrap();
            session_data.user_id = Some(user.id.to_string());
        }

        Ok(user.into())
    }

    async fn log_out<'a>(&self, ctx: &'a Context<'_>) -> Result<bool> {
        let gql_session = ctx.data::<GqlContext>().unwrap();
        let mut session_data = gql_session.lock().unwrap();
        session_data.user_id = None;

        Ok(true)
    }

    async fn user<'a>(&self, ctx: &'a Context<'_>) -> Result<OutputUser> {
        let id = {
            let gql_session = ctx.data::<GqlContext>().unwrap();
            let session_data = gql_session.lock().or(Err("cent get session"))?;
            let id = session_data.user_id.as_ref().ok_or("cent get user")?;
            Uuid::parse_str(id).or(Err("cent pars uuid"))?
        };

        let db = ctx.data::<Arc<Database>>().unwrap();
        let user = User::gt_by_id(db, id).await?;

        Ok(user.into())
    }
}

pub struct RootMutation;

#[Object]
impl RootMutation {
    async fn session(&self) -> Option<MutationSession> {
        Some(MutationSession::default())
    }
}

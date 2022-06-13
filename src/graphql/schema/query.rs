use crate::db::models::{OutputUser, User, InputFindUser};
use crate::graphql::context::GqlContext;
use async_graphql::{Context, Object, Result};
use mongodb::{bson::Uuid, Database};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Serialize, Debug, Default, Clone, Deserialize)]
pub struct QuerySession;

#[Object]
impl QuerySession {
    async fn user<'a>(&self, ctx: &'a Context<'_>) -> Result<Option<OutputUser>> {
        let id = {
            let gql_session = ctx.data::<GqlContext>().unwrap();
            let session_data = gql_session.lock()?;
            let id = session_data.get_user_id()?;

            if id.is_none() {
                return Ok(None);
            }

            id.unwrap()
        };

        let db = ctx.data::<Arc<Database>>().unwrap();
        let user = User::gt_by_id(db, id).await?;

        Ok(Some(user.into()))
    }

    async fn find_user<'a>(&self, ctx: &'a Context<'_>, data: InputFindUser) -> Result<Vec<OutputUser>> {
        let db = ctx.data::<Arc<Database>>().unwrap();
        let res = User::find_user(db, data).await?;

        Ok(res)
    }
}

pub struct RootQuery;
#[Object]
impl RootQuery {
    async fn session(&self) -> QuerySession {
        QuerySession::default()
    }
}

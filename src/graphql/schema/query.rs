use crate::db::models::{OutputUser, User};
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
            let session_data = gql_session.lock().or(Err("cent get session"))?;
            let id = session_data.user_id.as_ref();

            if id.is_none() {
                return Ok(None);
            }

            Uuid::parse_str(id.unwrap()).or(Err("cent pars uuid"))?
        };

        let db = ctx.data::<Arc<Database>>().unwrap();
        let user = User::gt_by_id(db, id).await?;

        Ok(Some(user.into()))
    }
}

pub struct RootQuery;
#[Object]
impl RootQuery {
    async fn session(&self) -> QuerySession {
        QuerySession::default()
    }
}

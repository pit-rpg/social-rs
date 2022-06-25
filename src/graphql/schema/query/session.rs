use crate::controllers::{InputFindUser, InputUserLogin, ControllerUser};
use crate::graphql::context::GqlContext;
use crate::error::GQLResult;
use crate::db::DBUser;
use async_graphql::{Context, Object};
use mongodb::{Database};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Serialize, Debug, Default, Clone, Deserialize)]
pub struct QuerySession;

#[Object]
impl QuerySession {
    async fn user<'a>(&self, ctx: &'a Context<'_>) -> GQLResult<Option<DBUser>> {
        let id = {
            let gql_session = ctx.data::<GqlContext>().unwrap();
            let session_data = gql_session.lock()?;
            let id = session_data.get_user_id();

            if id.is_none() {
                return Ok(None);
            }

            id.unwrap()
        };

        let db = ctx.data::<Arc<Database>>().unwrap();
        let user = ControllerUser::gt_by_id(db, id).await;

        Ok(user)
    }

    async fn find_user<'a>(
        &self,
        ctx: &'a Context<'_>,
        data: InputFindUser,
    ) -> GQLResult<Vec<DBUser>> {
        let db = ctx.data::<Arc<Database>>().unwrap();
        let res = ControllerUser::find_user(db, data).await?;

        Ok(res)
    }

    async fn get_user<'a>(
        &self,
        ctx: &'a Context<'_>,
        id: String,
    ) -> GQLResult<DBUser> {
        let db = ctx.data::<Arc<Database>>().unwrap();
        let res = ControllerUser::get_user(db, id).await?;

        Ok(res)
    }

    async fn register<'a>(&self, ctx: &'a Context<'_>, data: InputUserLogin) -> GQLResult<DBUser> {
        let user = {
            let db = ctx.data::<Arc<Database>>().unwrap();
            ControllerUser::register(db, data).await?
        };

        {
            let gql_session = ctx.data::<GqlContext>().unwrap();
            let mut session_data = gql_session.lock().unwrap();
            session_data.user_id = user.id.map(|id| id.to_string());
        }

        Ok(user)
    }

    async fn log_in<'a>(&self, ctx: &'a Context<'_>, data: InputUserLogin) -> GQLResult<DBUser> {
        let user = {
            let db = ctx.data::<Arc<Database>>().unwrap();
            ControllerUser::log_in(db, data).await?
        };

        {
            let gql_session = ctx.data::<GqlContext>().unwrap();
            let mut session_data = gql_session.lock().unwrap();
            session_data.user_id = user.id.map(|id| id.to_string());
        }


        Ok(user)
    }

    async fn log_out<'a>(&self, ctx: &'a Context<'_>) -> GQLResult<bool> {
        let gql_session = ctx.data::<GqlContext>().unwrap();
        let mut session_data = gql_session.lock().unwrap();
        session_data.user_id = None;

        Ok(true)
    }
}

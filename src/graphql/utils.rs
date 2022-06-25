// use crate::db::utils::to_object_id;
use async_graphql::Context;
use super::context::GqlContext;
use mongodb::bson::oid;
use crate::error::{Error, GQLResult};

pub fn get_user_id<'a>(
    ctx: &'a Context<'_>,
) -> GQLResult<oid::ObjectId> {
    let gql_session = ctx.data::<GqlContext>()?;
    let session_data = gql_session.lock()?;

    session_data.get_user_id().ok_or(Error::new_str("cen't get user id"))
}


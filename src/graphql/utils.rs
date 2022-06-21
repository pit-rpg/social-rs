// use crate::db::utils::to_object_id;
use async_graphql::Context;
use super::context::GqlContext;
use mongodb::bson::oid;
use crate::error::Error;

pub fn get_user_id<'a>(
    ctx: &'a Context<'_>,
) -> GQLResult<oid::ObjectId> {
    let gql_session = ctx.data::<GqlContext>().unwrap();
    let session_data = gql_session.lock()?;

    session_data.get_user_id().ok_or(Error::new("cen't get user id"))
}

pub type GQLResult<T, E = Error> = std::result::Result<T, E>;

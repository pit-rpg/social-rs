use super::context::*;
use super::schema::AppSchema;
use crate::db::RedisPool;
use actix_session::Session;
use actix_web::{web, HttpRequest, HttpResponse, Result};
use async_graphql::{
    http::{playground_source, GraphQLPlaygroundConfig},
    Data, Schema,
};
use graphql_actix_web_lib;
use graphql_actix_web_lib::{GraphQLRequest, GraphQLResponse, GraphQLSubscription};
use mongodb::Database;

pub async fn gql_playground() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(playground_source(
            GraphQLPlaygroundConfig::new("/").subscription_endpoint("/ws"),
        ))
}

pub async fn index(
    schema: web::Data<AppSchema>,
    redis_pool: web::Data<RedisPool>,
    mongo_db: web::Data<Database>,
    // req: HttpRequest,
    gql_request: GraphQLRequest,
    session: Session,
) -> Result<GraphQLResponse> {
    let request = gql_request.into_inner();

    let gql_context = ContextData::new(
        session.get("user_id")?,
    )
    .to_shared();

    let res = schema
        .execute(
            request
                .data(gql_context.clone())
                .data(redis_pool.into_inner())
                .data(mongo_db.into_inner()),
        )
        .await
        .into();

    {
        let data = gql_context.lock().unwrap();

        if data.user_id.is_none() {
            session.remove("user_id");
        } else {
            session.insert("user_id", &data.user_id)?;
        }
    }

    Ok(res)
}

pub async fn index_ws(
    schema: web::Data<AppSchema>,
    redis_pool: web::Data<RedisPool>,
    mongo_db: web::Data<Database>,
    session: Session,
    req: HttpRequest,
    payload: web::Payload,
) -> Result<HttpResponse> {
    let mut data = Data::default();

    let session_data = ContextData::new(
        session.get("user_id")?,
    );

    data.insert(session_data.to_shared());
    data.insert(redis_pool.into_inner());
    data.insert(mongo_db.into_inner());

    let res = GraphQLSubscription::new(Schema::clone(&*schema))
        .with_data(data)
        .start(&req, payload);

    res
}

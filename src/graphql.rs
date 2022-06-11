
use actix_web::{
    guard, http::header::HeaderMap, web, App,
    HttpRequest, HttpResponse, HttpServer, Result,
    Responder, ResponseError, Error, error,
    body::BoxBody
};
use async_graphql::{
    http::{playground_source, GraphQLPlaygroundConfig},
    Data, Schema, Context, Object, Subscription
};
// use actix_session::{Session, SessionMiddleware, storage::RedisActorSessionStore};

use std::time::Duration;
use std::sync::{Mutex, Arc, MutexGuard, PoisonError};
use async_graphql::EmptyMutation;

// use std::fmt::Display;
extern crate derive_more;
use derive_more::{Add, Display, From, Into, Unwrap};

// extern crate async_graphql_actix_web;
use graphql_actix_web_lib;
// use async_graphql_actix_web;
use graphql_actix_web_lib::{GraphQLRequest, GraphQLResponse, GraphQLSubscription};

// use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse, GraphQLSubscription};
pub type TokenSchema = Schema<QueryRoot, EmptyMutation, SubscriptionRoot>;

use deadpool::unmanaged::Pool;

use futures_util::Stream;
use serde::Deserialize;



use super::db::{RedisPool};

// #[derive(Default, Copy, Clone)]
// pub struct EmptySubscription;


#[derive(Clone, Debug, Default)]
pub struct SessionData {

    // #[derive(Clone, Debug, Display)]
    // #[display(fmt = "{:?}", user_id)]
    user_id: Option<String>
}

pub type GQL_Session = Arc<Mutex<SessionData>>;





pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn current_token<'a>(&self, ctx: &'a Context<'_>) -> Option<&'a str> {

        // let redis_pool = ctx.data_unchecked::<RedisPool>();
        // let user_id = actix_session.get::<i32>("user_id");
        {
            let gql_session = ctx.data::<GQL_Session>().unwrap();
            let mut session_data = gql_session.lock().unwrap();
            session_data.user_id = Some("SOME ID LOLO".to_string());
            println!("<><><>: {:?}", session_data);
        }

        Some("LOLOLO")
        // ctx.data_opt::<Token>().map(|token| token.0.as_str())
    }
}

// unsafe impl Send for SubscriptionRoot {}

pub struct SubscriptionRoot ;

#[Subscription]
impl SubscriptionRoot {
    async fn values(&self, ctx: &Context<'_>) -> actix_web::Result<impl Stream<Item = i32>, async_graphql::Error> {
        {
            let gql_session = ctx.data::<GQL_Session>()?;
            let session_data = gql_session.lock().unwrap();
            println!("<><><>: {:?}", session_data);
        }
        // if ctx.data::<Token>()?.0 != "123456" {
        //     return Err("Forbidden".into());
        // }
        Ok(futures_util::stream::once(async move { 10 }))
    }

    async fn interval(&self, #[graphql(default = 1)] n: i32) -> impl Stream<Item = i32> {
        let mut value = 0;
        async_stream::stream! {
            loop {
                futures_timer::Delay::new(Duration::from_secs(1)).await;
                value += n;
                yield value;
            }
        }
    }
}

// pub async fn on_connection_init(value: serde_json::Value) -> Result<Data, async_graphql::Error> {
//     #[derive(Deserialize)]
//     struct Payload {
//         token: String,
//     }

//     if let Ok(payload) = serde_json::from_value::<Payload>(value) {
//         let mut data = Data::default();
//         data.insert(Token(payload.token));
//         Ok(data)
//     } else {
//         Err("ERRRR<<>><><>><>>><<><><<".into())
//     }
// }


// pub struct SessionWarper (Arc<Mutex<Box<Session>>>);
// impl SessionWarper {
//     fn lock(&self) -> std::result::Result<MutexGuard<Box<Session>>, PoisonError<MutexGuard<Box<Session>>>>{
//         self.0.lock()
//     }

//     fn new(session : Session) -> Self {
//         SessionWarper(Arc::new(Mutex::new(Box::new(session))))
//     }
// }





pub async fn gql_playground() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(playground_source(
            GraphQLPlaygroundConfig::new("/").subscription_endpoint("/ws"),
        ))
}

// pub fn get_token_from_headers(headers: &HeaderMap) -> Option<Token> {
//     Some(Token("TOKK".into()))

//     // headers
//     //     .get("Token")
//     //     .and_then(|value| value.to_str().map(|s| Token(s.to_string())).ok())
// }

pub async fn index(
    schema: web::Data<TokenSchema>,
    req: HttpRequest,
    gql_request: GraphQLRequest,
    // session: Session,
) -> GraphQLResponse {
// ) -> GraphQLResponse {
    let request = gql_request.into_inner();
    // if let Some(token) = get_token_from_headers(req.headers()) {
    //     request = request.data(token);
    // }

    let mut data = Data::default();
    // let mut session_data = SessionData::default();

    // session_data.user_id = session.get("user_id").unwrap();

    // let gql_session = Arc::new(Mutex::new(session_data));

    // {

    //     println!(">>>> 1: {:?}", gql_session);

        // data.insert(gql_session.clone());
    // }

    let res = schema.execute(request).await.into();
    // let res = schema.execute(
    //     request.data(gql_session.clone())
    // ).await.into();

    // {
    //     let data = gql_session.lock().unwrap();

    //     // if let Some(id) = &data.user_id {
    //     //     session.insert("user_id", id).unwrap();
    //     // } else {
    //     //     session.remove("user_id");
    //     // }
    //     // let x = res.body();
    //     println!(">>>> 2: {:?}", data);

    // }

    res
    // let host = req.uri().host();
    // let path = req.uri().path();
    // let authority = req.uri().authority();
    // let path2 = req.path();
    // // let path2 = req.();

    // let res = HttpResponse
    //     ::Ok()
    //     .cookie(
    //         Cookie::build("name", "value")
    //             // .domain("www.rust-lang.org")
    //             .path("/")
    //             .secure(true)
    //             .http_only(true)
    //             .max_age(actix_web::cookie::time::Duration::days(1))
    //             .finish(),
    //     )
    //     .body(format!("LOLOLOLO {:?}, {:?}, {:?}, {:?}", host, path, authority, path2));
    //     // .finish();

    // Ok(res)
}

pub async fn index_ws(
    schema: web::Data<TokenSchema>,
    // session: Session,
    req: HttpRequest,
    payload: web::Payload,
) -> Result<HttpResponse> {
    let mut data = Data::default();
    let mut session_data = SessionData::default();

    // session_data.user_id = session.get("user_id")?;

    let gql_session: GQL_Session = Arc::new(Mutex::new(session_data));

    {

        println!(">>>> 1: {:?}", gql_session);

        data.insert(gql_session.clone());
    }
    // if let Some(token) = get_token_from_headers(req.headers()) {
    // }
    // let x = SessionWarper::new(session);
    // session.status()

    // let x = Arc::new(Mutex::new(session));

    // let foo = Box::new(session) as Box<Session + Send>;


    let res = GraphQLSubscription::new(Schema::clone(&*schema))
        .with_data(data)
        // .on_connection_init(on_connection_init)
        .start(&req, payload);
    // let res = GraphQLSubscription::new(Schema::clone(&*schema))
    //     .with_data(data)
    //     .on_connection_init(on_connection_init)
    //     .start(&req, payload);


    {
        // let x = res.body();
        println!(">>>> 2: {:?}", gql_session);

    }


    res
    // Ok(res)
}

pub fn get_schema() -> TokenSchema {
    let schema = Schema::build(QueryRoot, EmptyMutation, SubscriptionRoot)
        .finish();

    schema
}
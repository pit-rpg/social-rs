#[macro_use]
extern crate lazy_static;
extern crate validator;

mod db;
mod graphql;
mod controllers;

use actix_session::{storage::CookieSessionStore, SessionExt, SessionMiddleware};
use actix_web::{cookie::Key, dev::Service, middleware, web, App, HttpServer};
use mongodb::bson::oid::ObjectId;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let secret_key = Key::from(&[
        68, 7, 127, 34, 160, 144, 105, 241, 74, 45, 54, 104, 47, 64, 8, 61, 7, 116, 55, 186, 147,
        17, 113, 147, 35, 246, 232, 62, 136, 121, 66, 167, 71, 87, 177, 97, 19, 10, 20, 104, 217,
        202, 185, 153, 178, 52, 154, 96, 229, 15, 155, 202, 221, 38, 73, 154, 235, 9, 38, 219, 108,
        208, 158, 71,
    ]);

    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let (pool_redis_connection, _client_mongo, db_mongo) = db::connect::init(None).await;
    let schema = graphql::schema::get_schema();

    graphql::dump_schema_to_disk(&schema).await?;

    HttpServer::new(move || {
        App::new()
            // enable logger
            .wrap(middleware::Logger::default())
            .app_data(web::Data::new(db_mongo.clone()))
            .app_data(web::Data::new(pool_redis_connection.clone()))
            .wrap_fn(|req, srv| {
                let session = req.get_session();

                if session.get::<String>("session_uid").unwrap().is_none() {
                    session
                        .insert("session_uid", ObjectId::new().to_string())
                        .unwrap();
                }

                let fut = srv.call(req);

                async {
                    let res = fut.await?;
                    Ok(res)
                }
            })
            .wrap(SessionMiddleware::new(
                CookieSessionStore::default(),
                secret_key.clone(),
            ))
            .app_data(web::Data::new(schema.clone()))
            .service(web::resource("/").to(graphql::routes::index))
            .service(
                web::resource("/ws")
                    .guard(actix_web::guard::Get())
                    .guard(actix_web::guard::Header("upgrade", "websocket"))
                    .to(graphql::routes::index_ws),
            )
            .service(web::resource("/pg").to(graphql::routes::gql_playground))
    })
    .bind(("127.0.0.1", 3030))?
    .run()
    .await
}

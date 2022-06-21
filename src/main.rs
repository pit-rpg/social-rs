#[macro_use]
extern crate lazy_static;

extern crate validator;
extern crate dotenv;

use dotenv::dotenv;
mod db;
mod graphql;
mod controllers;
mod error;

use actix_session::{storage::CookieSessionStore, SessionExt, SessionMiddleware};
use actix_web::{cookie::Key, dev::Service, middleware, web, App, HttpServer};
use mongodb::bson::oid::ObjectId;
use std::env;
use actix_files as fs;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let app_host = env::var("APP_HOST").expect("env var APP_HOST is missing");
    let app_port = env::var("APP_PORT").expect("env var APP_PORT is missing");
    let app_port = app_port.parse::<u16>().expect("cent parse port env variable");

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

    println!("http://{}:{}", app_host, app_port);

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
            .service(web::resource("/graphql").to(graphql::routes::graphql))
            .service(
                web::resource("/ws")
                    .guard(actix_web::guard::Get())
                    .guard(actix_web::guard::Header("upgrade", "websocket"))
                    .to(graphql::routes::graphql_ws),
            )
            .service(web::resource("/pg").to(graphql::routes::gql_playground))
            .service(
                fs::Files::new("/", "dist")
                    .show_files_listing()
                    .use_last_modified(true)
                    .prefer_utf8(true)
                    .index_file("index.html")
            )
    })
    .bind((app_host, app_port))?
    .run()
    .await
}

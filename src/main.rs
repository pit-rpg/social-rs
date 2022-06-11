use actix_web::{
    cookie::{time, Cookie},
    dev::{Service, ServiceRequest, ServiceResponse},
    get,
    middleware,
    post,
    web,
    App,
    Error,
    HttpRequest,
    HttpResponse,
    HttpServer,
    Responder,
    Result,
};

mod db;
mod graphql;
// mod session;
// use actix_session::storage::SessionStore;

// async fn index(req: HttpRequest) -> &'static str {
//     println!("REQ: {:?}", req);
//     "Hello world!"
// }

// #[get("/echo")]
// async fn echo(req_body: String, session: Session) -> impl Responder {
//     let oldId = session.get::<String>("user_id").unwrap();
//     session.insert("user_id", "some user_id").unwrap();
//     let newId = session.get::<String>("user_id").unwrap();

//     format!("{:?}\n{:?}", oldId, newId)
// }

// async fn index(session: Session) -> actix_web::Result<&'static str, Error> {
//     // access the session state
//     if let Some(count) = session.get::<i32>("counter")? {
//         println!("SESSION value: {}", count);
//         // modify the session state
//         session.insert("counter", count + 1)?;
//     } else {
//         session.insert("counter", 1)?;
//     }

//     Ok("Welcome!")
// }

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let redis_connection_string = "127.0.0.1:6379";
    // let secret_key = Key::from(&[12;85]);
    // let secret_key = Key::generate();
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let (_client_mongo, db_mongo) = db::connect_mongo().await;
    let pool_redis_connection = db::connect_redis().await;
    let schema = graphql::get_schema();

    HttpServer::new(move || {
        App::new()
            // enable logger
            .wrap(middleware::Logger::default())
            .app_data(web::Data::new(db_mongo.clone()))
            .app_data(web::Data::new(pool_redis_connection.clone()))
            .wrap_fn(|req, srv| {
                let fut = srv.call(req);
                async {
                    let mut res = fut.await?;
                    let mut http_response = res.response_mut();

                    let is_session_id = http_response
                        .cookies()
                        .find(|i| i.name() == "session_id")
                        .is_some();

                    if !is_session_id {
                        let cookie = Cookie::build("name", "value")
                            // .domain("www.rust-lang.org")
                            .path("/")
                            .secure(true)
                            .http_only(true)
                            .max_age(time::Duration::days(1))
                            .finish();

                        http_response.add_cookie(&cookie);
                    }

                    // .insert(CONTENT_TYPE, HeaderValue::from_static("text/plain"));
                    Ok(res)
                }
            })
            // .service(echo)
            // .service(web::resource("/index.html").to(|| async { "Hello world!" }))
            .app_data(web::Data::new(schema.clone()))
            .service(web::resource("/").to(graphql::index))
            .service(
                web::resource("/ws")
                    .guard(actix_web::guard::Get())
                    .guard(actix_web::guard::Header("upgrade", "websocket"))
                    .to(graphql::index_ws),
            )
            .service(web::resource("/pg").to(graphql::gql_playground))
    })
    .bind(("127.0.0.1", 3030))?
    .run()
    .await
}

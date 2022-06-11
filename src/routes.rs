// use actix_redis::RedisSession;
use actix_redis::RedisSession;
use actix_session::Session;
use actix_web::{
    middleware, web,
    web::{get, post, resource},
    App, HttpResponse, HttpServer, Result,
};

use serde::{Deserialize, Serialize};

pub fn session_config(cfg: &mut web::ServiceConfig) {
    let private_key = actix_web::cookie::Key::generate();

    cfg.service(web::resource("/index.html").to(|| async { "Hello world!" }));
}

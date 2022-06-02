use mongodb::{Client, options::ClientOptions, Database};
// use mongodb::{Client, options::ClientOptions, error::Result};
// use std::error::Error;
use std::result::Result;
use std::boxed::Box;
use std::future;
use num_cpus;
use deadpool::unmanaged::Pool;
use redis::{Connection};

pub async fn connect_mongo() -> (Client, Database) {

    let mut client_options = ClientOptions
        ::parse("mongodb://root:example@localhost:27017")
        .await
        .expect("Can't connect to MongoDb");

    client_options.default_database =  Some("rust_social".to_string());
    client_options.app_name = Some("My App".to_string());

    let client = Client
        ::with_options(client_options)
        .expect("Can't create MongoDb client");

    let db = client.database("mydb");

    (client, db)
}

pub type RedisPool = Pool<Connection>;

pub async fn connect_redis() -> RedisPool {
    let items = num_cpus::get() * 2;

    let client = redis::Client::open("redis://127.0.0.1/")
        .expect("Can't connect to Redis");

    let connections: Vec<_> = (0..items)
        .map(|_| client.get_connection().unwrap())
        .collect();

    Pool::from(connections)
}

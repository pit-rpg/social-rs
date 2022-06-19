use super::{models::{DBUser, DBChat, DBChatMessage}, RedisPool};
use deadpool::unmanaged::Pool;
use mongodb::{options::ClientOptions, Client, Database};
use num_cpus;

pub async fn init(redis_pull_size: Option<usize>) -> (RedisPool, Client, Database) {
    let (client, db) = connect_mongo().await;
    let pool = connect_redis(redis_pull_size).await;

    create_indexes(&db).await;

    (pool, client, db)
}

pub async fn create_indexes(db: &Database) {
    DBUser::create_indexes(db).await;
    DBChat::create_indexes(db).await;
    DBChatMessage::create_indexes(db).await;
}

pub async fn connect_mongo() -> (Client, Database) {
    let mut client_options = ClientOptions::parse("mongodb://root:example@localhost:27017")
        .await
        .expect("Can't connect to MongoDb");

    client_options.default_database = Some("rust_social".to_string());
    client_options.app_name = Some("My App".to_string());

    let client = Client::with_options(client_options).expect("Can't create MongoDb client");

    let db = client.database("social-db");

    (client, db)
}

pub async fn connect_redis(pull_size: Option<usize>) -> RedisPool {
    let items = pull_size.unwrap_or(num_cpus::get() * 2);

    let client = redis::Client::open("redis://127.0.0.1/").expect("Can't connect to Redis");

    let connections: Vec<_> = (0..items)
        .map(|_| client.get_connection().expect("Cen't connect to redis"))
        .collect();

    Pool::from(connections)
}

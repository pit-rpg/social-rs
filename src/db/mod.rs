pub mod connect;
pub mod models;
pub mod validations;

use deadpool::unmanaged::Pool;
use mongodb::{Collection, Database};
use redis::Connection;

pub type RedisPool = Pool<Connection>;


pub trait CollectionUtils<T: Sized> {
    fn to_collection(db: &Database) -> Collection<T>;
    fn get_collection_name() -> &'static str;
}

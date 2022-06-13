pub mod connect;
pub mod models;
pub mod utils;
pub mod validations;

use deadpool::unmanaged::Pool;
use redis::Connection;

pub type RedisPool = Pool<Connection>;

pub mod connect;
pub mod utils;
pub mod validations;
mod models;

use deadpool::unmanaged::Pool;
use redis::Connection;

pub type RedisPool = Pool<Connection>;
pub use models::*;
use mongodb::{Collection, Database};
use std::time::{UNIX_EPOCH, SystemTime};

pub trait CollectionUtils<T: Sized> {
    fn to_collection(db: &Database) -> Collection<T>;
    fn get_collection_name() -> &'static str;
}

pub fn date_now() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()
}
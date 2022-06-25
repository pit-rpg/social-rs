use mongodb::{Collection, Database, bson::oid::ObjectId};
use std::{time::{UNIX_EPOCH, SystemTime}, collections::HashMap};

pub trait CollectionUtils<T: Sized> {
    fn to_collection(db: &Database) -> Collection<T>;
    fn get_collection_name() -> &'static str;
}

pub fn date_now() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()
}

pub fn map_string_to_id(item: &Option<String>) -> Option<ObjectId> {
    match item {
        Some(data) => ObjectId::parse_str(data).ok(),
        None => None
    }
}

pub fn map_id_to_string(item: &Option<ObjectId>) ->  Option<String>{
    item.as_ref().map(|i| i.to_string() )
}

pub fn map_id_hash_map(item: &HashMap<String, ObjectId>) -> HashMap<String, String>{
    item
        .into_iter()
        .map(|(id1, id2)| (id1.clone(), id2.to_string()))
        .collect::<HashMap<String, String>>()
}

pub fn map_option_id_hesh_map(item: &Option<HashMap<String, ObjectId>>) -> Option<HashMap<String, String>>{
    item.as_ref().map(|i| map_id_hash_map(&i))
}


use std::sync::{Arc, Mutex, MutexGuard};
use mongodb::{bson::oid::ObjectId};
use async_graphql::{Result};

#[derive(Clone, Debug, Default)]
pub struct ContextData {
    pub user_id: Option<String>,
}

impl ContextData {
    pub fn new(user_id: Option<String>) -> Self {
        ContextData {
            user_id,
        }
    }

    pub fn get_user_id(&self) -> Result<Option<ObjectId>> {
        if self.user_id.is_none() {return Ok(None)}

        let id = ObjectId::parse_str(self.user_id.as_ref().unwrap()).or(Err("cent pars ObjectId"))?;

        Ok(Some(id))
    }

    pub fn to_shared(self) -> GqlContext {
        GqlContext(Arc::new(Mutex::new(self)))
    }
}

#[derive(Clone)]
pub struct GqlContext(Arc<Mutex<ContextData>>);

impl GqlContext {
    pub fn lock(&self) -> std::result::Result<MutexGuard<ContextData>, &'static str> {
        self.0.lock().or(Err("cent get ContextData"))
    }
}
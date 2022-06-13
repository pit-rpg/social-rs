use std::sync::{Arc, Mutex, MutexGuard};
use mongodb::{bson::Uuid, Database};
use async_graphql::{Context, Object, Result};

#[derive(Clone, Debug, Default)]
pub struct ContextData {
    pub user_id: Option<String>,
    session_uid: String,
}

impl ContextData {
    pub fn new(session_uid: String, user_id: Option<String>) -> Self {
        ContextData {
            session_uid,
            user_id,
        }
    }

    pub fn get_session_uid<'a>(&'a self) -> &'a str {
        &self.session_uid
    }

    pub fn get_user_id(&self) -> Result<Option<Uuid>> {
        if self.user_id.is_none() {return Ok(None)}

        let id = Uuid::parse_str(self.user_id.as_ref().unwrap()).or(Err("cent pars uuid"))?;

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
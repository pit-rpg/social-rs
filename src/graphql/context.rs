use std::sync::{Arc, Mutex};

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
}

pub type GqlContext = Arc<Mutex<ContextData>>;

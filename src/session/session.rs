// use actix_utils::future::{ready, Ready};

use actix_web::{
    dev::{Extensions, Payload, ServiceRequest, ServiceResponse},
    error::Error,
    FromRequest, HttpMessage, HttpRequest,
};

use serde::{de::DeserializeOwned, Serialize};

use std::{
    cell::{Ref, RefCell},
    collections::HashMap,
    mem,
    rc::Rc,
    sync::{Mutex, Arc, MutexGuard, PoisonError},
};

use async_lock::{
    RwLock,
};

#[derive(Debug, Clone)]
struct SessionInner {
    state: HashMap<String, String>,
    changed: bool,
}

impl Default for SessionInner {
    fn default() -> Self {
        SessionInner {
            state: HashMap::default(),
            changed: true,
        }
    }
}

pub struct Session(Arc<RwLock<SessionInner>>);


impl Session {
    pub async fn get<T: DeserializeOwned>(&self, key: &str) -> Result<Option<T>, serde_json::Error> {
        let inner = self.0.read().await;

        if let Some(val_str) = &inner.state.get(key) {
            Ok(Some(serde_json::from_str(val_str)?))
        } else {
            Ok(None)
        }
    }

    pub async fn entries(&self) -> HashMap<String, String> {
        let inner = self.0.read().await;

        inner.state.clone()
    }

    pub async fn changed(&self) -> bool {
        let inner = self.0.read().await;
        inner.changed
    }

    pub async fn insert(
        &self,
        key: impl Into<String>,
        value: impl Serialize,
    ) -> Result<(), serde_json::Error> {
        let mut inner = self.0.write().await;

        inner.changed = true;
        let val = serde_json::to_string(&value)?;
        inner.state.insert(key.into(), val);

        Ok(())
    }

    pub async fn remove(&self, key: &str) -> Option<String> {
        let mut inner = self.0.write().await;

        inner.changed = true;
        inner.state.remove(key)
    }

    pub async fn clear(&self) {
        let mut inner = self.0.write().await;
        inner.changed = true;
        inner.state.clear()
    }

    pub(crate) async fn set_session(
        req: &mut ServiceRequest,
        data: impl IntoIterator<Item = (String, String)>,
    ) {
        let session = Session::get_session(&mut *req.extensions_mut());
        let mut inner = session.0.write().await;
        inner.state.extend(data);
    }

    pub(crate) fn get_session(extensions: &mut Extensions) -> Session {
        if let Some(s_impl) = extensions.get::<Arc<RwLock<SessionInner>>>() {
            return Session(s_impl.clone());
        }

        let inner = Arc::new(RwLock::new(SessionInner::default()));
        extensions.insert(inner.clone());

        Session(inner)
    }
}

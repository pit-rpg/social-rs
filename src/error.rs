// use crate::db::utils::to_object_id;
use mongodb::bson::oid;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::result;

#[derive(Serialize, Debug, Clone, Deserialize)]
pub struct Error {
    msg: &'static str,
    log: Option<String>,
}

impl Error {
    pub fn new(msg: &'static str) -> Self {
        Error {msg, log: None }

    }
    pub fn new_with_log(msg: &'static str, log: String) -> Self {
        Error {msg, log: Some(log)}
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(log) = &self.log {
            println!("{}", log);
        }

        write!(f, "{}", self.msg)
    }
}

impl From<oid::Error> for Error {
    fn from(_: oid::Error) -> Error {
        Error::new("cent parse ObjectId")
    }
}

impl From<&'static str> for Error {
    fn from(err: &'static str) -> Error {
        Error::new(err)
    }
}

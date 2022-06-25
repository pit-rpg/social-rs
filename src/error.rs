// use crate::db::utils::to_object_id;
use mongodb::bson::oid;
use serde::{Serialize};
use std::fmt;

#[derive(Serialize, Debug, Clone)]
pub enum ErrMsg {
    Str(&'static str),
    String(String),
}

impl fmt::Display for ErrMsg {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ErrMsg::Str(x) => {write!(f, "{}", x)},
            ErrMsg::String(x) => {write!(f, "{}", x)},
        }
    }
}


#[derive(Serialize, Debug, Clone)]

pub struct Error {
    msg: ErrMsg,
    log: Option<String>,
}

impl Error {
    pub fn new(msg: String, log: Option<String>) -> Self {
        Error {msg: ErrMsg::String(msg), log}
    }

    pub fn new_str(msg: &'static str) -> Self {
        Error {msg: ErrMsg::Str(msg), log: None}
    }

    pub fn new_str_log(msg: &'static str, log: Option<String>) -> Self {
        Error {msg: ErrMsg::Str(msg), log}
    }
}

pub type GQLResult<T, E = Error> = std::result::Result<T, E>;

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
        Error::new_str("cent parse ObjectId")
    }
}

impl From<&'static str> for Error {
    fn from(err: &'static str) -> Error {
        Error::new_str(err)
    }
}

impl From<async_graphql::Error> for Error {
    fn from(err: async_graphql::Error) -> Error {
        Error::new(format!("{}", err.message), None)
    }
}

impl From<mongodb::error::Error> for Error {
    fn from(err: mongodb::error::Error) -> Error {
        Error::new(format!("{}", err), None)
    }
}

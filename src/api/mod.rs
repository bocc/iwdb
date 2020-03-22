// TODO don't
#![allow(dead_code)]

use serde::{Deserialize, Serialize};

// inserts
#[derive(Serialize, Debug)]
pub enum RespInsertStatus {
    Inserted,
    AlreadyExisted,
    Invalid,
}

#[derive(Serialize, Debug)]
pub struct RespInsert {
    pub word: String,
    pub status: RespInsertStatus,
}

#[derive(Deserialize, Debug)]
pub struct ReqInsert {
    pub add: String,
}

// queries
#[derive(Serialize, Debug)]
pub enum RespQueryStatus {
    Found,
    NotFound,
    Invalid,
}

#[derive(Serialize, Debug)]
pub struct RespQuery {
    pub word: String,
    pub status: RespInsertStatus,
}

#[derive(Deserialize, Debug)]
pub struct ReqQuery {
    pub word: String,
}

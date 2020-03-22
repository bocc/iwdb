// TODO don't
#![allow(dead_code)]

use hyper::{Body, Request, Response, StatusCode};
use serde::{Deserialize, Serialize};

use std::collections::HashSet;
use std::sync::{Arc, RwLock};

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

// TODO these functions below should be implementations of a trait

// own everything (perhaps a better name would be `edward`)
pub async fn add_word(req: Request<Body>, words: Arc<RwLock<HashSet<String>>>) -> Response<Body> {
    let body = hyper::body::to_bytes(req.into_body()).await;

    let body = match body {
        Ok(body) => {
            let request: Result<ReqInsert, _> = serde_json::from_slice(&body);

            if let Ok(request) = request {
                let mut words = words.write().unwrap();

                if words.contains(&request.add) {
                    Body::from(
                        serde_json::to_string(&RespInsertStatus::AlreadyExisted)
                            .expect("response serialization failed"),
                    )
                } else {
                    words.insert(request.add);
                    Body::from(
                        serde_json::to_string(&RespInsertStatus::Inserted)
                            .expect("response serialization failed"),
                    )
                }
            } else {
                Body::from(
                    serde_json::to_string(&RespInsertStatus::Inserted)
                        .expect("response serialization failed"),
                )
            }
        }
        Err(_) => Body::from("wrong body"),
    };

    Response::new(body)
}

pub async fn query_word(req: Request<Body>, words: Arc<RwLock<HashSet<String>>>) -> Response<Body> {
    let body = hyper::body::to_bytes(req.into_body()).await;

    let body = match body {
        Ok(body) => {
            let query: Result<ReqQuery, _> = serde_json::from_slice(&body);

            if let Ok(query) = query {
                let words = words.read().unwrap();

                if words.contains(&query.word) {
                    Body::from("contains")
                } else {
                    Body::from("doesn't contain")
                }
            } else {
                Body::from("wrong question")
            }
        }
        Err(_) => Body::from("wrong body"),
    };

    Response::new(body)
}

pub async fn default_response() -> Response<Body> {
    Response::builder()
        .status(StatusCode::METHOD_NOT_ALLOWED)
        .body(Body::empty())
        .expect("Could not create response.")
}

// TODO return more detailed resons enum
fn validate_word(s: &str) -> bool {
    // cannot be empty and cannot contain whitespace (here it is assumed to be trimmed)
    !(s.is_empty() || s.chars().any(|c| c.is_whitespace()))
}

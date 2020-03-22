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
    // mapping errors to &str is not elegant
    let response = hyper::body::to_bytes(req.into_body())
        .await
        .map_err(|_| "couldn't read request body")
        .and_then(|body| {
            serde_json::from_slice::<ReqInsert>(&body).map_err(|_| "couldn't parse request")
        })
        .map(|req| {
            let word = req.add.trim();

            if word_is_valid(word) {
                // TODO only read lock for containment check?
                let mut words = words.write().expect("lock was poisoned");

                if words.contains(word) {
                    RespInsertStatus::AlreadyExisted
                } else {
                    words.insert(word.to_string());
                    RespInsertStatus::Inserted
                }
            } else {
                RespInsertStatus::Invalid
            }
        })
        .map(|insert_status| {
            Response::new(Body::from(
                serde_json::to_string(&insert_status).expect("response serialization failed"),
            ))
        })
        .unwrap_or_else(|err| {
            // let's just blame the user
            Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body(Body::from(err))
                .expect("Could not create response.")
        });

    response
}

// TODO a lot of duplication here
pub async fn query_word(req: Request<Body>, words: Arc<RwLock<HashSet<String>>>) -> Response<Body> {
    let response = hyper::body::to_bytes(req.into_body())
        .await
        .map_err(|_| "couldn't read request body")
        .and_then(|body| {
            serde_json::from_slice::<ReqQuery>(&body).map_err(|_| "couldn't parse request")
        })
        .map(|req| {
            let word = req.word.trim();

            if word_is_valid(word) {
                let words = words.read().expect("lock was poisoned");

                if words.contains(word) {
                    RespQueryStatus::Found
                } else {
                    RespQueryStatus::NotFound
                }
            } else {
                RespQueryStatus::Invalid
            }
        })
        .map(|query_status| {
            Response::new(Body::from(
                serde_json::to_string(&query_status).expect("response serialization failed"),
            ))
        })
        .unwrap_or_else(|err| {
            // let's just blame the user
            Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body(Body::from(err))
                .expect("Could not create response.")
        });

    response
}

pub async fn default_response() -> Response<Body> {
    Response::builder()
        .status(StatusCode::METHOD_NOT_ALLOWED)
        .body(Body::empty())
        .expect("Could not create response.")
}

// TODO return more detailed reasons enum?
fn word_is_valid(s: &str) -> bool {
    // cannot be empty and cannot contain whitespace (here it is assumed to be trimmed)
    !(s.is_empty() || s.chars().any(|c| c.is_whitespace()))
}

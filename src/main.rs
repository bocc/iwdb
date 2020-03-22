#![feature(type_name_of_val)]
// init words set from local file and/or web api
mod init;
// types for json hadling in requests and responses
mod api;
// types required to parse iwdb.toml
mod configuration;
mod server;

use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Request, Response, Server, StatusCode};
// use serde::{Serialize, Deserialize};
use std::convert::Infallible;
use std::sync::{Arc, RwLock};

use api::ReqQuery;

#[tokio::main]
async fn main() {
    let config = configuration::parse_configuration("./iwdb.toml");

    // create a set and fill a hashset with some values (before wrapping it in Arc)
    let mut words = std::collections::HashSet::new();

    init::insert_words(&mut words, &config).await;

    let words = Arc::new(RwLock::new(words));

    let make_service = make_service_fn(move |_conn| {
        let words = words.clone();

        async move {
            Ok::<_, Infallible>(service_fn(move |req: Request<Body>| {
                let words = words.clone();
                async move {
                    let resp = match (req.method(), req.uri().path()) {
                        (&Method::GET, "/query") => {
                            // it seems problematic to access our word set from an async
                            // context, this is why we block here. :[
                            let body = hyper::body::to_bytes(req.into_body()).await;

                            let resp = match body {
                                Ok(body) => {
                                    let query: Result<ReqQuery, _> = serde_json::from_slice(&body);

                                    if let Ok(query) = query {
                                        let words = words.read().unwrap();

                                        if words.contains(&query.word) {
                                            Response::new(Body::from("contains"))
                                        } else {
                                            Response::new(Body::from("doesn't contain"))
                                        }
                                    } else {
                                        Response::new(Body::from("wrong question"))
                                    }
                                }
                                Err(_) => Response::new(Body::from("wrong body")),
                            };

                            resp
                        }

                        (&Method::POST, "/add") => {
                            let mut words = words.write().unwrap();
                            let _a1 = words.insert("aldsjfk".to_string());
                            Response::new(Body::from("add"))
                        }

                        _ => Response::builder()
                            .status(StatusCode::METHOD_NOT_ALLOWED)
                            .body(Body::empty())
                            .expect("Could not create response."),
                    };

                    Ok::<_, Infallible>(resp)
                }
            }))
        }
    });

    let addr = std::net::SocketAddr::new(config.server.ip, config.server.port);

    let server = Server::bind(&addr)
        .serve(make_service)
        .with_graceful_shutdown(server::shutdown_signal());

    println!("Listening on {}", &addr);

    if let Err(e) = server.await {
        eprintln!("Server error: {}", e);
    }
}

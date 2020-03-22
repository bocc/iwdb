#![feature(type_name_of_val)]
// init words set from local file and/or web api
mod init;
// types for json hadling in requests and responses
mod api;
// types required to parse iwdb.toml
mod configuration;
mod server;

use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Request, Server};

use std::convert::Infallible;
use std::sync::{Arc, RwLock};

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
                        (&Method::GET, "/query") => api::query_word(req, words).await,

                        (&Method::POST, "/add") => api::add_word(req, words).await,

                        _ => api::default_response().await,
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

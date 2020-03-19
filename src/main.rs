// init words set from local file and/or web api
mod init;
// types for json hadling
mod api;
// types required to parse iwdb.toml
mod configuration;
mod server;

use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Response, Server, StatusCode};
use std::convert::Infallible;
use std::net::SocketAddr;
use std::sync::{Arc, RwLock};

#[tokio::main]
async fn main() {
    let config = configuration::parse_configuration("./iwdb.toml");

    // fill a hashset with some values (before wrapping it in Arc)
    let mut words = std::collections::HashSet::new();

    init::insert_words(&mut words, &config).await;

    let words = Arc::new(RwLock::new(words));

    let make_service = make_service_fn(move |_conn| {
        let _words = words.clone();

        async move {
            // This is the `Service` that will handle the connection.
            // `service_fn` is a helper to convert a function that
            // returns a Response into a `Service`.
            Ok::<_, Infallible>(service_fn(move |req| {
                let resp = match (req.method(), req.uri().path()) {
                    (&Method::GET, "/query") => {
                        Ok::<_, Infallible>(Response::new(Body::from("query")))
                    }
                    (&Method::PUT, "/add") => Ok(Response::new(Body::from("add"))),
                    _ => Ok(Response::builder()
                        .status(StatusCode::METHOD_NOT_ALLOWED)
                        .body(Body::empty())
                        .expect("Could not create response.")),
                };

                async move { resp }
            }))
        }
    });

    let addr = SocketAddr::new(config.server.ip, config.server.port);

    let server = Server::bind(&addr)
        .serve(make_service)
        .with_graceful_shutdown(server::shutdown_signal());

    println!("Listening on {}", &addr);

    if let Err(e) = server.await {
        eprintln!("Server error: {}", e);
    }
}

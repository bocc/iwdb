// init words set from local file and/or web api
mod init;
// types for json hadling in requests and responses
mod api;
// types required to parse iwdb.toml
mod configuration;
mod server;

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Request, Server};

use std::convert::Infallible;

use kudzu::raw::SkipList;
use lazy_static::lazy_static;

lazy_static! {
    // we are storing u64 hashes only, as the standard hasher works with those
    static ref SKIPLIST: SkipList<u64> = {
        let s = SkipList::new();
        s
    };
}

#[tokio::main]
async fn main() {
    let config = configuration::parse_or_default("./iwdb.toml");

    init::insert_words(&SKIPLIST, &config).await;

    // with a skiplist, returning a closure require no locking, cloning or
    // ownership transfer
    let make_service = make_service_fn(|_conn| async {
        Ok::<_, Infallible>(service_fn(|req: Request<Body>| async {
            let resp = match (req.method(), req.uri().path()) {
                (&Method::GET, "/query") => api::query_word(req, &SKIPLIST).await,

                (&Method::POST, "/add") => api::add_word(req, &SKIPLIST).await,

                _ => api::default_response(),
            };

            Ok::<_, Infallible>(resp)
        }))
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

fn calculate_hash<T: Hash + ?Sized>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

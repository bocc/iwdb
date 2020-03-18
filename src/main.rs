mod init;

use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Response, Server, StatusCode};
use std::convert::Infallible;
use std::net::SocketAddr;
use std::sync::{Arc, RwLock};

// what to do when CTRL-C is pressed
async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("Could not set CTRL+C signal handler.");
    println!("iwdb shutting down.");
}

#[tokio::main]
async fn main() {
    let addr = "https://random-word-api.herokuapp.com/word?number=10";

    // fill a hashset with some values (before wrapping it in Arc)
    let mut words = std::collections::HashSet::new();

    let res = init::from_file::add_words(&mut words, "./assets/init_words.txt");

    match res {
        Ok(n) => println!("Inserted {} words from {}", n, "./assets/init_words.txt"),
        Err(e) => eprintln!("Faild to insert words from file because {}", e),
    }

    let res = init::from_web_api::add_words(&mut words, addr).await;

    match res {
        Ok(n) => println!("Inserted {} words from {}", n, addr),
        Err(e) => eprintln!("Faild to insert words from web API because {}", e),
    }

    let words = Arc::new(RwLock::new(words));

    let make_service = make_service_fn(move |_conn| {
        let _words = words.clone();

        async move {
            // This is the `Service` that will handle the connection.
            // `service_fn` is a helper to convert a function that
            // returns a Response into a `Service`.
            Ok::<_, Infallible>(service_fn(move |req| {
                let resp = match (req.method(), req.uri().path()) {
                    (&Method::GET, "/query") => 
                        Ok::<_,Infallible>(Response::new(Body::from("query"))),
                    (&Method::PUT, "/add") => {
                        Ok(Response::new(Body::from("add"))) 
                    },
                    _ => Ok(Response::builder()
                        .status(StatusCode::METHOD_NOT_ALLOWED)
                        .body(Body::empty())
                        .expect("Could not create response.")),
                };
            
                async move { resp }
            }))
        }
    });

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    let server = Server::bind(&addr)
        .serve(make_service)
        .with_graceful_shutdown(shutdown_signal());

    println!("Listening on {}", &addr);

    if let Err(e) = server.await {
        eprintln!("Server error: {}", e);
    }
}

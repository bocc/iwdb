// what to do when CTRL-C is pressed
pub(crate) async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("Could not set CTRL+C signal handler.");
    println!("iwdb shutting down.");
}

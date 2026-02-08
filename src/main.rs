use std::net::TcpListener;
use zero2prod::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let default_port = "8000";
    let listener = TcpListener::bind(format!("127.0.0.1:{}", &default_port))
        .unwrap_or_else(|_| panic!("Failed to bind port {}", &default_port));
    run(listener)?.await
}

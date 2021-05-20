use std::net::TcpListener;

use zero2prod::run;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    // Listen TCP in localhost:8000
    let listener = TcpListener::bind("127.0.0.1:8000")
        .expect("Failed to bind listener");

    run(listener)?.await
}
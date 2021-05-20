use std::net::TcpListener;

use zero2prod::run;
use zero2prod::configuration::get_configuration;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let configuration = get_configuration().expect("Failed to read configuration");

    let addres = format!("127.0.0.1:{}", configuration.application_port);

    // Listen TCP in localhost:8000
    let listener = TcpListener::bind(addres)
        .expect("Failed to bind listener");

    run(listener)?.await
}
use std::net;
use actix_web::{App, HttpServer};
use actix_web::web::ServiceConfig;


pub async fn run_server(listener: net::TcpListener, router_config: fn(&mut ServiceConfig)) -> std::io::Result<()> {
    let listen_addr = listener.local_addr().unwrap().to_string();
    let server = HttpServer::new(move || {
        App::new()
            .configure(router_config)
    })
        .listen(listener)?
        .run();


    println!("Server is listening on {}", listen_addr);
    return server.await;
}
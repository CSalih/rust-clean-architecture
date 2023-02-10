use std::net::TcpListener;

use actix_web::web;

use crate::common::http::actix_server;

pub mod common;
pub mod user;

pub fn configure_routes(config: &mut web::ServiceConfig) {
    user::infrastructure::controller::user_controller::add_routes(config)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let listen_address = "0.0.0.0:8080";
    let listener = TcpListener::bind(listen_address).expect("Failed to bind random port");

    actix_server::run_server(listener, configure_routes).await
}
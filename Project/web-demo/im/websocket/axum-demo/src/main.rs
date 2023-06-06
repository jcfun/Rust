use axum::{routing::get, Extension, Router, Server};
use axum_demo::{ws_handler, ChatState};
use std::net::{IpAddr, Ipv6Addr, SocketAddr};

#[tokio::main]
async fn main() {
    let addr = SocketAddr::new(IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 0)), 3000);
    let app = Router::new().route(
        "/ws",
        get(ws_handler).layer(Extension(ChatState::default())),
    );
    println!("listening on {}", addr);
    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

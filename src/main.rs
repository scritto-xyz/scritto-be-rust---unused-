use std::net::SocketAddr;
use axum::{routing::get, Router};
use std::env;

#[tokio::main]
async fn main() {
    let port_result = env::var("PORT");
    let port = match port_result  {
        Ok(p) => p.parse::<u16>().unwrap(),
        Err(_) => panic!("Port not provided"),
    };
    let app = Router::new().route("/", get(handler));
    let addr = SocketAddr::from(([0,0,0,0], port));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handler() -> &'static str {
let str = "Hello, world!";
    return str;
}

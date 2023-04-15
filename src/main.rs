mod models;
mod controllers;
mod config;
mod error;

use std::net::SocketAddr;
use std::env;
use axum::{routing::{get, post}, Router};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use tracing;
use std::string::ToString;
use mysql::{Pool};
use tower_http::cors::{Any, CorsLayer};
use crate::config::clients::mysql_client_get_pool;


#[derive(Clone)]
pub struct AppState {
    conn_pool: Pool
}

#[tokio::main]
async fn main() {

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            env::var("RUST_LOG").unwrap_or_else(|_| "scritto=debug".into())
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();
    let port = env::var("PORT").unwrap_or("8080".to_string()).parse::<u16>().unwrap();

    let cors = CorsLayer::new().allow_origin(Any);

    let conn = mysql_client_get_pool().await.expect("unable to connect to db");

    let state = AppState {
        conn_pool: conn
    };

    let app = Router::new()
        .route("/test", get(hello_world_handler))
        .route("/register", post(controllers::auth::register))
        .with_state(state)
        .layer(cors);

    let addr = SocketAddr::from(([0,0,0,0], port));

    tracing::debug!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("failed to start server");
}

async fn hello_world_handler() -> &'static str {
let str = "Hello, world!";
    return str;
}

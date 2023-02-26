use std::net::SocketAddr;
use std::env;
use axum::{
    routing::{get, post},
    Router,
    response::{Json},
};
use serde::Deserialize;
use chrono::prelude::*;
use serde_json::{Value, json};

#[tokio::main]
async fn main() {
    let port_result = env::var("PORT");
    let port = match port_result  {
        Ok(p) => p.parse::<u16>().unwrap(),
        Err(_) => panic!("Port not provided"),
    };

    let app = Router::new()
        .route("/", get(hello_world_handler))
        .route("/user", post(create_user_handler));

    let addr = SocketAddr::from(([0,0,0,0], port));
    let addr_str = addr.to_string();
    println!("=======Server running at address: {addr_str}=======");

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}


#[derive(Deserialize)]
enum UserType {
    Artist,
    Client
}

#[derive(Deserialize)]
struct User {
    id: i32,
    first_name: String,
    last_name: String,
    email: String,
    password: String,
    country: String,
    state: String,
    city: String,
    user_type: UserType,
    created_ts: DateTime<Utc>,
    updated_ts: DateTime<Utc>,
}


async fn hello_world_handler() -> &'static str {
let str = "Hello, world!";
    return str;
}

async fn create_user_handler(Json(user): Json<User>) -> Json<Value> {
    let response_json: Json<Value> = Json(json!({
        "id": 1,
    }));

    return response_json;
}

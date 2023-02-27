use std::net::SocketAddr;
use std::env;
use std::env::VarError;
use axum::{
    routing::{get, post},
    Router,
    response::{Json},
};
use serde::Deserialize;
use chrono::prelude::*;
use http::StatusCode;
use mysql::*;
use mysql::prelude::*;
use serde_json::{Value, json};
use tracing_subscriber;
use tracing;
use std::string::ToString;
use strum_macros::Display;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let port = env::var("PORT").unwrap_or("8080".to_string()).parse::<u16>().unwrap();

    let app = Router::new()
        .route("/test", get(hello_world_handler))
        .route("/users", post(create_user_handler));

    let addr = SocketAddr::from(([0,0,0,0], port));
    tracing::debug!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[derive(Deserialize, Display, Debug, PartialEq, Eq)]
enum UserType {
    Artist,
    Client
}

#[derive(Deserialize)]
struct CreateUser {
    first_name: String,
    last_name: String,
    email: String,
    password: String,
    country: String,
    state: String,
    city: String,
    user_type: UserType
}

#[derive(Debug, PartialEq, Eq)]
struct GetUser {
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

async fn get_conn() -> Result<PooledConn> {
    let url = env::var("MYSQL_URL").expect("MYSQL URL env var not found");
    let pool = Pool::new(&*url)?;
    let conn = pool.get_conn();
    return conn;
}

async fn create_user_handler(Json(user): Json<CreateUser>) -> (StatusCode, Json<Value>) {
    let mut conn = get_conn().await.unwrap();

    // insert new user
    conn.exec_drop("
        INSERT INTO railway.users (
            first_name,
            last_name,
            email,
            password,
            country,
            state,
            city,
            user_type
        )
        VALUES (?,?,?,?,?,?,?,?)
    ", (
        user.first_name,
        user.last_name,
        user.email,
        user.password,
        user.country,
        user.state,
        user.city,
        user.user_type.to_string(),
    )).unwrap();

    let id: Option<i32> = conn.query_first("SELECT LAST_INSERT_ID()")
        .expect("Returns the last inserted id");
    let response_json: Json<Value> = Json(json!({
        "id": id,
    }));

    return (StatusCode::CREATED, response_json);
}

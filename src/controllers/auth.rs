use std::env;
use axum::{
    response::{Json},
};
use http::StatusCode;
use mysql::*;
use mysql::prelude::*;
use serde_json::{Value, json};

use crate::{
    models
};

async fn get_conn() -> Result<PooledConn> {
    let url = env::var("MYSQL_URL").expect("MYSQL URL env var not found");
    let pool = Pool::new(&*url)?;
    let conn = pool.get_conn();
    return conn;
}

pub async fn register(Json(user): Json<models::users::CreateUser>) -> (StatusCode, Json<Value>) {
    let mut conn = get_conn().await.expect("unable to connect to database");

    // TODO ==> add logic for checking for existing email in table already

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
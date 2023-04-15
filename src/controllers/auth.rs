use axum::{extract::{State, Json}};
use mysql::{params, Row};
use mysql::Params::Positional;
use mysql::prelude::Queryable;
use serde_json::{Value, json};

use crate::{AppState, models};
use crate::error::error::AppError;

pub async fn register(
    State(app_state): State<AppState>,
    Json(user): Json<models::users::CreateUser>,
) -> Result<Json<Value>, AppError> {
    let conn_result = app_state.conn_pool.get_conn();
    let mut conn = match conn_result {
        Ok(c) => c,
        Err(_) => return Err(AppError::InternalServerError),
    };


    // check if email is already in db
    if user.email.is_empty() {
        return Err(AppError::InternalServerError);
    }

    let filtered_user_id: Option<Row> = conn.exec_first(
        "SELECT id
        FROM railway.users
        WHERE email = ?
    ", (&user.email,)
    ).map_err(|_| AppError::InternalServerError)?;

    if let Some(_) = filtered_user_id {
        return Err(AppError::UserAlreadyExists);
    }

    // insert new user
    let _ = conn.exec_drop("
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
    )).map_err(|_| AppError::InternalServerError)?;

    if conn.affected_rows() < 1 {
        return Err(AppError::InternalServerError);
    }

    let id: Option<i32> = conn.query_first("SELECT LAST_INSERT_ID()")
        .expect("Returns the last inserted id");
    let response_json: Json<Value> = Json(json!({
        "id": id,
    }));

    Ok(response_json)
}
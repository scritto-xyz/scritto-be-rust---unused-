use axum::{extract::{State, Json}};
use mysql::{Row};
use sea_orm::ActiveValue::Set;
use sea_orm::{ColumnTrait, InsertResult};
use sea_orm::EntityTrait;
use serde_json::{Value, json};
use entity::users;
use crate::{AppState, utils::jwt, config::clients};
use crate::error::error::AppError;

pub async fn register(
    State(app_state): State<AppState>,
    Json(user): Json<User>,
) -> Result<Json<Value>, AppError> {
    let mut conn = app_state.conn;

    // check if email is already in db
    if user.email.is_empty() {
        return Err(AppError::InternalServerError);
    }

    // let user: User = Users::find()
    //     .filter(entity::users::Column::Email.eq(user.email.as_ref()))
    //     .one(conn)
    //     .await?;

    // let filtered_user: User = Users::find()
    //     .filter(entity::users::Column::Email.eq(user.email.as_ref()))
    //     .one(conn.pipe_as_ref())
    //     .await?;

    if let Some(_) = user {
        return Err(AppError::UserAlreadyExists);
    }

    // add user to db
    let new_user_am = entity::users::ActiveModel {
        first_name: Set(user.first_name),
        last_name: Set(user.last_name),
        email: Set(user.email),
        password: Set(user.password),
        country: Set(user.country),
        state: Set(user.state),
        city: Set(user.city),
        user_type: Set(user.user_type),
        ..Default::default()
    };

    let res: InsertResult<i32> = Users::insert(new_user_am).exec(conn.pipe_as_ref()).await?;

    // check if user was added
    if res.rows_affected != 1 || !res.last_insert_id  {
        return Err(AppError::InternalServerError);
    }

    let response_json: Json<Value> = Json(json!({
        "id": res.last_insert_id,
    }));

    Ok(response_json)
}
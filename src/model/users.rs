use serde::Deserialize;
use chrono::prelude::*;
use mysql::*;
use strum_macros::Display;

#[derive(Deserialize, Display, Debug, PartialEq, Eq)]
pub enum UserType {
    Artist,
    Client
}

#[derive(Deserialize)]
pub struct CreateUser {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
    pub country: String,
    pub state: String,
    pub city: String,
    pub user_type: UserType
}

#[derive(Debug, PartialEq, Eq)]
pub struct GetUser {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
    pub country: String,
    pub state: String,
    pub city: String,
    pub user_type: UserType,
    pub created_ts: DateTime<Utc>,
    pub updated_ts: DateTime<Utc>,
}
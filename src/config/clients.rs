use std::env;
use dotenvy::dotenv;
use sea_orm::{Database, DatabaseConnection};


pub async fn mysql_client_get_pool() -> DatabaseConnection {
    dotenv().ok();
    let url = env::var("DATABASE_URL").expect("DATABASE_URL env var not found");
    Database::connect(url).await.expect("Unable to establish db connection")
}
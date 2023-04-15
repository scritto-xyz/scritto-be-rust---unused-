use std::env;
use mysql::{Result, Pool};


pub async fn mysql_client_get_pool() -> Result<Pool> {
    let url = env::var("MYSQL_URL").expect("MYSQL URL env var not found");
    return Pool::new(&*url);
}
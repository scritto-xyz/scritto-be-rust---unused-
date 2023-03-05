use std::env;
use mysql::{Result, Pool, PooledConn};


pub async fn mysql_client_get_conn() -> Result<PooledConn> {
    let url = env::var("MYSQL_URL").expect("MYSQL URL env var not found");
    let pool = Pool::new(&*url)?;
    let conn = pool.get_conn();
    return conn;
}
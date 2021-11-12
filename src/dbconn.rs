use sqlx::{MySql, Pool};
use sqlx::mysql::MySqlPoolOptions;
pub async fn do_connect() -> Pool<MySql> {

    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect("mysql://test:new@localhost/test").await;
    pool.unwrap()
}
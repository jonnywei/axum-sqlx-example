use sqlx::{Error, MySql, Pool};
use sqlx::mysql::MySqlQueryResult;
use sqlx::FromRow;
use serde::{Serialize,Deserialize};

#[derive(FromRow,  Serialize, Deserialize)]
pub struct  ShortLink {
    pub id :u32,
    pub url:String,
}


pub async fn create_shortlink(pool : &Pool<MySql> , url: &str) ->Result<MySqlQueryResult,Error> {

    sqlx::query(
        r#"
            insert into short_links(`url`)
            values (?)
        "#
    ).bind(url).execute(pool).await

}

pub async fn get_short_links(pool : &Pool<MySql> , id: u32) -> Result<ShortLink,Error>{
    sqlx::query_as::<_,ShortLink>(r#"
        SELECT * FROM short_links
            WHERE id = ?
    "#).bind(id).fetch_one(pool).await
}
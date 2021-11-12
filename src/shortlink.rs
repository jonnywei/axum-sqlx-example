use axum::extract::Extension;
use axum::http::StatusCode;
use axum::{extract, Json};
use axum::response::IntoResponse;
use sqlx::{MySql, Pool};
use serde::{Deserialize, Serialize};
use crate::model;
use crate::model::ShortLink;

#[derive(Debug,Serialize,Deserialize,Clone)]
pub struct  CreateShortLinkRequest {
    pub url:String,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateShortLinkResp {
    pub ok: bool,
}

pub async fn create_shortlink (Json(req) : Json<CreateShortLinkRequest>,
                               Extension(pool): Extension<Pool<MySql>>)
    -> impl IntoResponse{

    println!("{:#?}",req);
    match model::create_shortlink(&pool,&req.url).await {
        Ok(result) => {
            println!("{:#?}",result);
            (StatusCode::OK, Json(CreateShortLinkResp{ok:true}))
        }
        Err(e) => {
            println!("{:#?}",e);
            (StatusCode::OK, Json(CreateShortLinkResp{ok:false}))
        }
    }

}


pub async fn get_short_link(extract::Path(id): extract::Path<u32>
                            ,Extension(pool): Extension<Pool<MySql>>)
    -> impl IntoResponse {

    match model::get_short_links(&pool,id).await {
        Ok(shortlink) => {
            (StatusCode::OK, Json((shortlink)))
        },
        Err(err) => {
            println!("{:#?}",err);
            (StatusCode::OK, Json(model::ShortLink{ id, url: "".to_string() }))
        }

    }

}
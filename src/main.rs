mod dbconn;
mod model;
mod shortlink;

use axum::{response::Html, routing::get, Router, AddExtensionLayer};
use std::net::SocketAddr;
use axum::routing::post;
use sqlx::{MySql, Pool};

#[tokio::main]
async fn main() {
    let pool = dbconn::do_connect().await;
    let app = app(pool);
    let addr = SocketAddr::from(([127,0,0,1],5678));
    println!("Hello, world!");
    axum::Server::bind(&addr).serve(app.into_make_service()).await.unwrap();

}

fn app(pool : Pool<MySql>) -> Router {
    let app = Router::new().route("/", get(handler))
        .nest("/api",short_link_router())
        .layer(AddExtensionLayer::new(pool));
    app

}

fn short_link_router() -> Router{

    Router::new().route("/create_shortlink",post(shortlink::create_shortlink))
        .route("/get/:id",get(shortlink::get_short_link))
}

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, world!</h1>")
}
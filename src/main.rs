use axum::{Router, response::Html, routing::get};
use presentation::general::html::final_html;
use std::fs;
use tokio::net::TcpListener;

mod presentation;
mod schema;

#[tokio::main]
async fn main() {
    println!("Serving at localhost:9876");
    axum::serve(
        TcpListener::bind("0.0.0.0:9876").await.unwrap(),
        Router::new().route(
            "/",
            get(|| async {
                let page = final_html();
                fs::write("index.html", &page).unwrap();
                Html(page)
            }),
        ),
    )
    .await
    .unwrap();
}

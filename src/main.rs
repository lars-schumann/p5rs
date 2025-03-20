use axum::{Router, response::Html, routing::get};
use std::net::SocketAddr;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    let router = Router::new()
        .route(
            "/",
            get(|| async { Html(std::fs::read_to_string("static/index.html").unwrap()) }),
        )
        .nest_service("/static", ServeDir::new("static"));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server running at http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, router).await.unwrap();
}

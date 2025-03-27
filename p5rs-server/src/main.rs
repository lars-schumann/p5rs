use axum::{Router, http::StatusCode, response::IntoResponse, response::Response, routing::get};
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    let cors = CorsLayer::new().allow_origin(Any);

    let app = Router::new()
        .route("/code/example/js", get(serve_code_example_js))
        .route("/code/example/wasm", get(serve_code_example_wasm))
        .fallback_service(ServeDir::new("./frontend/dist"))
        .layer(cors);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn serve_code_example_js() -> impl IntoResponse {
    match std::fs::read_to_string("./code/example_1/pkg/p5rs_wasm.js") {
        Ok(content) => Response::builder()
            .status(StatusCode::OK)
            .header("Content-Type", "application/javascript")
            .body(content)
            .unwrap(),
        Err(_) => Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body("JavaScript file p5rs_wasm.js not found".into())
            .unwrap(),
    }
}

async fn serve_code_example_wasm() -> impl IntoResponse {
    match std::fs::read("./code/example_1/pkg/p5rs_wasm_bg.wasm") {
        Ok(content) => Response::builder()
            .status(StatusCode::OK)
            .header("Content-Type", "application/wasm")
            .body::<axum::body::Body>(content.into())
            .unwrap(),
        Err(_) => Response::builder()
            .status(StatusCode::NOT_IMPLEMENTED)
            .body("wasm file p5rs_wasm_bg.wasm not found".into())
            .unwrap(),
    }
}

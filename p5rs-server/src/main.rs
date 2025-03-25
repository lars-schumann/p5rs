use axum::{
    Json, Router, http::StatusCode, response::Html, response::IntoResponse, response::Response,
    routing::get,
};
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};
use tower_http::services::ServeDir;
// NEW
mod types;
use types::Person;
#[tokio::main]
async fn main() {
    let cors = CorsLayer::new().allow_origin(Any);

    let app = Router::new()
        .route("/people", get(get_people))
        .route("/dynamic-script.js", get(serve_dynamic_js))
        .route("/scriptA.js", get(serve_script_a_js))
        .route("/p5rs_wasm_bg.wasm", get(serve_module_wasm))
        .fallback_service(ServeDir::new("../p5rs-client/dist"))
        .layer(cors);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn get_people() -> impl IntoResponse {
    let people = vec![
        Person {
            name: String::from("Person A"),
            age: 36,
            favourite_food: Some(String::from("Pizza")),
        },
        Person {
            name: String::from("Person B"),
            age: 5,
            favourite_food: Some(String::from("Broccoli")),
        },
        Person {
            name: String::from("Person C"),
            age: 100,
            favourite_food: None,
        },
    ];

    (StatusCode::OK, Json(people))
}

async fn serve_dynamic_js() -> impl IntoResponse {
    match std::fs::read_to_string("../p5rs-wasm/test.js") {
        Ok(content) => Response::builder()
            .status(StatusCode::OK)
            .header("Content-Type", "application/javascript")
            .body(content)
            .unwrap(),
        Err(_) => Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body("JavaScript file not found".into())
            .unwrap(),
    }
}

async fn serve_script_a_js() -> impl IntoResponse {
    match std::fs::read_to_string("./pkg/p5rs_wasm.js") {
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

async fn serve_module_wasm() -> impl IntoResponse {
    match std::fs::read("./pkg/p5rs_wasm_bg.wasm") {
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

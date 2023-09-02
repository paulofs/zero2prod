use axum::{extract::Path, http::StatusCode, response::IntoResponse, routing::get, Router};

async fn greet(Path(name): Path<String>) -> impl IntoResponse {
    format!("Hello, {name}!")
}
/// Teste
async fn root() -> &'static str {
    "Hello, World!"
}

/// Return `200 OK` if the API is running and is accessible
async fn health_check() -> StatusCode {
    StatusCode::OK
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root))
        .route("/:name", get(greet))
        .route("/health_check", get(health_check));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

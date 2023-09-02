use axum::{extract::Path, response::IntoResponse, routing::get, Router};

async fn greet(Path(name): Path<String>) -> impl IntoResponse {
    format!("Hello, {name}!")
}
/// Teste
async fn root() -> &'static str {
    "Hello, World!"
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root))
        .route("/:name", get(greet));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

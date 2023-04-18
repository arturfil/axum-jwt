use axum::{Json, response::IntoResponse, Router, routing::get};


pub async fn health_checker_handler() -> impl IntoResponse {
    const MESSAGE: &str = "JWT Authentication in Rust usin Axum, Postgres, and SQLX";

    let json_response = serde_json::json!({
        "status": "success",
        "message": MESSAGE
    });
    Json(json_response)
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/api/healthchecker", get(health_checker_handler));

    println!("🚀 Server started successfully");
    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

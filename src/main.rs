mod config;


use redis::Client;
use std::sync::Arc;
use axum::{Json, response::IntoResponse, Router, routing::get};
use config::Config;
use dotenv::dotenv;
use sqlx::{Postgres, Pool, postgres::PgPoolOptions};

pub struct AppState {
    db: Pool<Postgres>,
    env: Config,
    redis_client: Client,
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let config = Config::init();
    
    let pool = match PgPoolOptions::new()
        .max_connections(10)
        .connect(&config.database_url)
        .await
    {
        Ok(pool) => {
            print!("✅ Connection to the database is successful!");
            pool
        }
        Err(err) => {
            print!("🔥 Failed to connect to the database: {:?}", err);
                std::process::exit(1);
        }
    };

    let redis_client = match Client::open(config.redis_url.to_owned()) {
        Ok(client) => {
            println!("✅ Connection to redis is successful!")
        }
        Err(e) => {
            println!("🔥 Error connecting to Redis: {}", e);
            std::process::exit(1);
        }
    };

    let app = Router::new()
        .route("/api/healthchecker", get(health_checker_handler))
        .with_state(Arc::new(AppState {
            db: pool.clone(),
            env: config.clone(),
            redis_client: redis_client.clone(),
        }));

    println!("🚀 Server started successfully");
    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

pub async fn health_checker_handler() -> impl IntoResponse {
    const MESSAGE: &str = "Rust and Axum Framework: JWT Access and Refresh Tokens";

    let json_response = serde_json::json!({
        "status": "success",
        "message": MESSAGE
    });

    Json(json_response)
}


use axum::{
    routing::get,
    http::StatusCode,
    handler::Handler,
    Json,
    Router,
};
use serde::Deserialize;

use log::info;
use env_logger;

use tokio;

async fn fallback() -> (StatusCode, &'static str) {
    (StatusCode::OK, "All right!")
}

async fn api_echoing() -> (StatusCode, Json<serde_json::Value>) {
    info!("API fallback");
    (
        StatusCode::ACCEPTED,
        Json(serde_json::json!({
            "status": "ok",
            "message": "All right!" 
        })),
    )
}

#[derive(Deserialize)]
struct DefaultPayload {
    path: String,
    payload: String,
    method: String,
    query: String,
}


#[tokio::main]
async fn main() {
    let env = env_logger::Env::default()
    .filter_or("LOG_LEVEL", "debug")
    .write_style_or("LOG_STYLE", "always");
    env_logger::init_from_env(env);

    // build our application with a single route
    let app = Router::new().fallback(api_echoing);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
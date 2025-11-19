mod config;
mod error;
mod models;
mod routes;
use tower_http::cors::{CorsLayer};
use axum::http::{HeaderValue, Method};

#[tokio::main]
async fn main() {
    // Initialize Config
    let state = config::AppState::new();

    // Setup CORS 
    let cors = CorsLayer::new()
        .allow_origin(
            state.frontend_url
                .parse::<HeaderValue>()
                .expect("Invalid FRONTEND_URL config")
        ) 
        .allow_methods([Method::POST, Method::OPTIONS])
        .allow_headers([axum::http::header::CONTENT_TYPE]);

    // Build Router
    let app = routes::create_router(state)
        .layer(cors);

    // Run Server
    let port = std::env::var("PORT").unwrap_or_else(|_| "3001".to_string());
    let addr = format!("0.0.0.0:{}", port);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    
    println!("->> SERVER RUNNING on {}", addr);
    axum::serve(listener, app).await.unwrap();
}
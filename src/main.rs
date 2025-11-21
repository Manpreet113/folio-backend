mod auth_middleware;
mod config;
mod db;
mod error;
mod models;
mod routes;

use axum::http::{HeaderValue, Method, header};
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    // Initialize DB
    let pool = db::init_db().await;

    // Initialize Config
    let state = config::AppState::new(pool);

    // Setup CORS
    let cors = CorsLayer::new()
        .allow_origin(
            state
                .frontend_url
                .parse::<HeaderValue>()
                .expect("Invalid FRONTEND_URL config"),
        )
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::DELETE,
            Method::OPTIONS,
        ])
        .allow_headers([header::CONTENT_TYPE, header::AUTHORIZATION]);

    // Build Router
    let app = routes::create_router(state).layer(cors);

    // Run Server
    let port = std::env::var("PORT").unwrap_or_else(|_| "3001".to_string());
    let addr = format!("0.0.0.0:{}", port);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    println!("->> SERVER RUNNING on {}", addr);
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<std::net::SocketAddr>(),
    )
    .await
    .unwrap();
}

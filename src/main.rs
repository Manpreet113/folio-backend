use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::post,
    Json, Router,
};
use reqwest;
use serde::{Deserialize, Serialize};
use std::env;
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};

// App configs, like API keys
#[derive(Clone)]
struct AppState {
    resend_api_key: String,
    to_email: String,
    from_email: String,
}

// JSON payload from frontend
#[derive(Deserialize, Debug)]
struct ContactPayload {
    name: String,
    email: String,
    message: String,
}

// JSON payload sent to Resend
#[derive(Serialize, Debug)]
struct ResendPayload<'a> {
    from: &'a str,
    to: &'a [&'a str],
    subject: String,
    html: String,
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().expect("Failed to load .env file. Make sure it exists.");

    let resend_api_key =
        env::var("RESEND_API_KEY").expect("RESEND_API_KEY must be set in .env");
    let to_email = env::var("TO_EMAIL").expect("TO_EMAIL must be set in .env");
    let from_email = env::var("FROM_EMAIL").expect("FROM_EMAIL must be set in .env");

    // Store keys in application state
    let app_state = AppState {
        resend_api_key,
        to_email,
        from_email,
    };

    // CORS layer setup bkc
    let cors = CorsLayer::new()
        .allow_origin(Any) 
        .allow_methods(Any)
        .allow_headers(Any);

    // Defining routes
    let app = Router::new()
        .route("/api/contact", post(contact_handler))
        .with_state(app_state) // Make the state available to handlers
        .layer(cors); // Apply the CORS middleware

    // Server Ronner
    let addr = SocketAddr::from(([127, 0, 0, 1], 3001));
    println!("->> RUST BACKEND LISTENING on http://{}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

/// Handler for the POST /api/contact route
async fn contact_handler(
    State(state): State<AppState>, // Extract the app state
    Json(payload): Json<ContactPayload>,
) -> Result<impl IntoResponse, AppError> {
    
    println!("->> RECEIVED contact form submission: {:?}", payload);

    // Email formating
    let subject = format!("New Contact Form Message from {}", payload.name);
    let html_body = format!(
        "<h1>New Portfolio Message!</h1>
         <p><strong>From:</strong> {}</p>
         <p><strong>Email:</strong> <a href='mailto:{}'>{}</a></p>
         <hr>
         <p><strong>Message:</strong></p>
         <pre>{}</pre>",
        payload.name, payload.email, payload.email, payload.message
    );

    // Make Resend payload
    let resend_payload = ResendPayload {
        from: &state.from_email,
        to: &[&state.to_email],
        subject: subject,
        html: html_body,
    };

    // Send the request to Resend
    let client = reqwest::Client::new();
    let res = client
        .post("https://api.resend.com/emails")
        .bearer_auth(&state.resend_api_key) // Use the API key
        .json(&resend_payload)
        .send()
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?; // Handle reqwest error

    // Handle Resend's response
    if res.status().is_success() {
        println!("->> SUCCESS: Email sent via Resend");
        let success_response = serde_json::json!({ "message": "Message sent successfully!" });
        Ok((StatusCode::OK, Json(success_response)))
    } else {
        let status = res.status();
        let error_text = res.text().await.unwrap_or_default();
        println!("->> ERROR: Resend API error: {} - {}", status, error_text);
        Err(AppError::Resend(format!(
            "Resend API failed: {}",
            error_text
        )))
    }
}

// Just a custom error type
enum AppError {
    Internal(String),
    Resend(String),
}

// IntoResponse for AppError to turn errors into a proper HTTP response
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::Internal(msg) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Internal server error: {}", msg),
            ),
            AppError::Resend(msg) => (
                StatusCode::BAD_GATEWAY, // 502 if Resend fails
                format!("Error sending email: {}", msg),
            ),
        };

        let body = Json(serde_json::json!({ "error": error_message }));
        (status, body).into_response()
    }
}
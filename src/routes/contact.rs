use axum::{extract::State, Json, http::StatusCode, response::IntoResponse};
use std::sync::Arc;
use validator::Validate;
use html_escape::encode_text;

use crate::{
    config::AppState,
    error::AppError,
    models::{ContactPayload, ResendPayload},
};

pub async fn contact_handler(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<ContactPayload>,
) -> Result<impl IntoResponse, AppError> {
    if let Err(e) = payload.validate() {
        return Err(AppError::ValidationError(e.to_string()));
    }

    println!("->> RECEIVED contact form submission: {:?}", payload.email);

    let safe_name = encode_text(&payload.name);
    let safe_email = encode_text(&payload.email);
    let safe_message = encode_text(&payload.message);
    let subject = format!("New Portfolio Message from {}", safe_name);
    let html_body = format!(
        "<h1>New Portfolio Message!</h1>
         <p><strong>From:</strong> {}</p>
         <p><strong>Email:</strong> <a href='mailto:{}'>{}</a></p>
         <hr>
         <p><strong>Message:</strong></p>
         <pre>{}</pre>",
        safe_name, safe_email, safe_email, safe_message
    );

    let resend_payload = ResendPayload {
        from: &state.from_email,
        to: &[&state.to_email],
        subject,
        html: html_body,
    };

    let client = reqwest::Client::new();
    let res = client
        .post("https://api.resend.com/emails")
        .bearer_auth(&state.resend_api_key)
        .json(&resend_payload)
        .send()
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    if res.status().is_success() {
        Ok((StatusCode::OK, Json(serde_json::json!({ "message": "Message sent successfully!" }))))
    } else {
        let error_text = res.text().await.unwrap_or_default();
        Err(AppError::Resend(format!("Resend API failed: {}", error_text)))
    }
}
use axum::{routing::post, Router};
use std::sync::Arc;
use tower_governor::{governor::GovernorConfigBuilder, GovernorLayer};
use crate::config::AppState;

mod contact;

pub fn create_router(state: Arc<AppState>) -> Router {
    // Rate Limit: Max 2 requests per 5 seconds per IP
    let governor_conf = GovernorConfigBuilder::default()
        .per_second(5)
        .burst_size(2)
        .finish()
        .unwrap();

    Router::new()
        .route("/api/contact", post(contact::contact_handler))
        .layer(GovernorLayer {
            config: Arc::new(governor_conf),
        })
        .with_state(state)
}
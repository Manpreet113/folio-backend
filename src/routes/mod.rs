use crate::config::AppState;
use axum::{Router, routing::get, routing::post};
use std::sync::Arc;
use tower_governor::{GovernorLayer, governor::GovernorConfigBuilder};

mod auth;
mod contact;
mod content;

use crate::auth_middleware::auth_middleware;
use axum::middleware;

pub fn create_router(state: Arc<AppState>) -> Router {
    // Rate Limit: Max 2 requests per 5 seconds per IP
    let governor_conf = GovernorConfigBuilder::default()
        .per_second(5)
        .burst_size(2)
        .finish()
        .unwrap();

    let protected_routes = Router::new()
        .route("/api/skills", post(content::create_skill))
        .route(
            "/api/skills/:id",
            axum::routing::delete(content::delete_skill),
        )
        .route("/api/projects", post(content::create_project))
        .route(
            "/api/projects/:id",
            axum::routing::delete(content::delete_project),
        )
        .layer(middleware::from_fn(auth_middleware));

    let public_routes = Router::new()
        .route("/api/contact", post(contact::contact_handler))
        .route("/api/skills", get(content::get_skills))
        .route("/api/projects", get(content::get_projects))
        .route("/api/login", post(auth::login_handler));

    Router::new()
        .merge(public_routes)
        .merge(protected_routes)
        .layer(GovernorLayer {
            config: Arc::new(governor_conf),
        })
        .with_state(state)
}

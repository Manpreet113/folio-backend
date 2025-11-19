use std::env;
use std::sync::Arc;
use crate::db::DbPool;

#[derive(Clone)]
pub struct AppState {
    pub resend_api_key: String,
    pub to_email: String,
    pub from_email: String,
    pub frontend_url: String,
    pub db: DbPool,
}

impl AppState {
    pub fn new(db: DbPool) -> Arc<Self> {
        dotenvy::dotenv().ok();

        let resend_api_key = env::var("RESEND_API_KEY").expect("RESEND_API_KEY must be set");
        let to_email = env::var("TO_EMAIL").expect("TO_EMAIL must be set");
        let from_email = env::var("FROM_EMAIL").expect("FROM_EMAIL must be set");
        let frontend_url = env::var("FRONTEND_URL").expect("FRONTEND_URL must be set");

        Arc::new(Self {
            resend_api_key,
            to_email,
            from_email,
            frontend_url,
            db,
        })
    }
}
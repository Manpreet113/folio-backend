use axum::{extract::State, Json, http::StatusCode, response::IntoResponse};
use std::sync::Arc;
use crate::{config::AppState, models::Skill, error::AppError};

pub async fn get_skills(
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, AppError> {
    let skills = sqlx::query_as::<_, Skill>("SELECT * FROM skills ORDER BY id DESC")
        .fetch_all(&state.db)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    Ok((StatusCode::OK, Json(skills)))
}
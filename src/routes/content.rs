use crate::config::AppState;
use crate::error::AppError;
use crate::models::content::{CreateProjectRequest, CreateSkillRequest, Project, Skill};
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use uuid::Uuid;

use std::sync::Arc;

// Skills
pub async fn get_skills(State(state): State<Arc<AppState>>) -> Result<Json<Vec<Skill>>, AppError> {
    let skills = sqlx::query_as::<_, Skill>("SELECT * FROM skills ORDER BY created_at DESC")
        .fetch_all(&state.db)
        .await
        .map_err(|e| AppError::InternalServerError(e.to_string()))?;

    Ok(Json(skills))
}

pub async fn create_skill(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateSkillRequest>,
) -> Result<Json<Skill>, AppError> {
    let skill = sqlx::query_as::<_, Skill>(
        "INSERT INTO skills (name, category, proficiency, icon) VALUES ($1, $2, $3, $4) RETURNING *"
    )
    .bind(payload.name)
    .bind(payload.category)
    .bind(payload.proficiency)
    .bind(payload.icon)
    .fetch_one(&state.db)
    .await
    .map_err(|e| AppError::InternalServerError(e.to_string()))?;

    Ok(Json(skill))
}

pub async fn delete_skill(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    sqlx::query("DELETE FROM skills WHERE id = $1")
        .bind(id)
        .execute(&state.db)
        .await
        .map_err(|e| AppError::InternalServerError(e.to_string()))?;

    Ok(StatusCode::NO_CONTENT)
}

// Projects
pub async fn get_projects(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<Project>>, AppError> {
    let projects = sqlx::query_as::<_, Project>("SELECT * FROM projects ORDER BY created_at DESC")
        .fetch_all(&state.db)
        .await
        .map_err(|e| AppError::InternalServerError(e.to_string()))?;

    Ok(Json(projects))
}

pub async fn create_project(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateProjectRequest>,
) -> Result<Json<Project>, AppError> {
    let project = sqlx::query_as::<_, Project>(
        "INSERT INTO projects (title, description, tech_stack, image_url, github_url, demo_url) VALUES ($1, $2, $3, $4, $5, $6) RETURNING *"
    )
    .bind(payload.title)
    .bind(payload.description)
    .bind(&payload.tech_stack)
    .bind(payload.image_url)
    .bind(payload.github_url)
    .bind(payload.demo_url)
    .fetch_one(&state.db)
    .await
    .map_err(|e| AppError::InternalServerError(e.to_string()))?;

    Ok(Json(project))
}

pub async fn delete_project(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    sqlx::query("DELETE FROM projects WHERE id = $1")
        .bind(id)
        .execute(&state.db)
        .await
        .map_err(|e| AppError::InternalServerError(e.to_string()))?;

    Ok(StatusCode::NO_CONTENT)
}

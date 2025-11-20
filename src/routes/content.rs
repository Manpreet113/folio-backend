use crate::config::AppState;
use crate::error::AppError;
use crate::models::content::{
    CreateProjectRequest, CreateSkillRequest, Project, Skill, UpdateProjectRequest,
    UpdateSkillRequest,
};
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

pub async fn update_skill(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateSkillRequest>,
) -> Result<Json<Skill>, AppError> {
    let skill = sqlx::query_as::<_, Skill>(
        "UPDATE skills SET name = $1, category = $2, proficiency = $3, icon = $4 WHERE id = $5 RETURNING *"
    )
    .bind(payload.name)
    .bind(payload.category)
    .bind(payload.proficiency)
    .bind(payload.icon)
    .bind(id)
    .fetch_optional(&state.db)
    .await
    .map_err(|e| AppError::InternalServerError(e.to_string()))?
    .ok_or_else(|| AppError::InternalServerError(format!("Skill with id {} not found", id)))?;

    Ok(Json(skill))
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

pub async fn update_project(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateProjectRequest>,
) -> Result<Json<Project>, AppError> {
    let project = sqlx::query_as::<_, Project>(
        "UPDATE projects SET title = $1, description = $2, tech_stack = $3, image_url = $4, github_url = $5, demo_url = $6 WHERE id = $7 RETURNING *"
    )
    .bind(payload.title)
    .bind(payload.description)
    .bind(&payload.tech_stack)
    .bind(payload.image_url)
    .bind(payload.github_url)
    .bind(payload.demo_url)
    .bind(id)
    .fetch_optional(&state.db)
    .await
    .map_err(|e| AppError::InternalServerError(e.to_string()))?
    .ok_or_else(|| AppError::InternalServerError(format!("Project with id {} not found", id)))?;

    Ok(Json(project))
}

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, FromRow, Validate)]
pub struct Skill {
    #[serde(skip_deserializing)]
    pub id: Uuid,
    #[validate(length(min = 1, message = "Name cannot be empty"))]
    pub name: String,
    pub category: String,
    pub proficiency: String,
    pub icon: Option<String>,
    #[serde(skip_deserializing)]
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateSkillRequest {
    #[validate(length(min = 1, message = "Name cannot be empty"))]
    pub name: String,
    pub category: String,
    pub proficiency: String,
    pub icon: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateSkillRequest {
    #[validate(length(min = 1, message = "Name cannot be empty"))]
    pub name: String,
    pub category: String,
    pub proficiency: String,
    pub icon: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, FromRow, Validate)]
pub struct Project {
    #[serde(skip_deserializing)]
    pub id: Uuid,
    #[validate(length(min = 1, message = "Title cannot be empty"))]
    pub title: String,
    pub description: String,
    pub tech_stack: Vec<String>,
    pub image_url: Option<String>,
    pub github_url: Option<String>,
    pub demo_url: Option<String>,
    #[serde(skip_deserializing)]
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateProjectRequest {
    #[validate(length(min = 1, message = "Title cannot be empty"))]
    pub title: String,
    pub description: String,
    pub tech_stack: Vec<String>,
    pub image_url: Option<String>,
    pub github_url: Option<String>,
    pub demo_url: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateProjectRequest {
    #[validate(length(min = 1, message = "Title cannot be empty"))]
    pub title: String,
    pub description: String,
    pub tech_stack: Vec<String>,
    pub image_url: Option<String>,
    pub github_url: Option<String>,
    pub demo_url: Option<String>,
}

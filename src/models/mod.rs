use serde::{Deserialize, Serialize};
use validator::Validate;
use sqlx::FromRow;

#[derive(Deserialize, Debug, Validate)]
pub struct ContactPayload {
    #[validate(length(min = 1, message = "Name cannot be empty"))]
    pub name: String,

    #[validate(email(message = "Invalid email address"))]
    pub email: String,

    #[validate(length(min = 10, message = "Message must be at least 10 characters long"))]
    pub message: String,
}

#[derive(Serialize, Debug)]
pub struct ResendPayload<'a> {
    pub from: &'a str,
    pub to: &'a [&'a str],
    pub subject: String,
    pub html: String,
}

#[derive(Serialize, FromRow, Debug)]
pub struct Skill {
    pub id: i64,
    pub name: String,
    pub category: String,
    pub status: String,
    pub icon: Option<String>,
}
use serde::{Deserialize, Serialize};
use crate::models::user::User;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Recipe {
    pub id: i32,
    pub title: String,
    pub ingredients: Vec<String>,
    pub instructions: String,
    pub comments: Vec<String>,
    pub created_by: User,
}
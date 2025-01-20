use serde::{Deserialize, Serialize};
use crate::dto::user_dto::UserDto;

#[derive(Serialize, Deserialize, Debug)]
pub struct RecipeDto {
    pub id: i32,
    pub title: String,
    pub ingredients: Vec<String>,
    pub instructions: String,
    pub comments: Vec<String>,
    pub created_by: UserDto,
}
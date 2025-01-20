use crate::assembler::user_assembler::{from_user_dto, to_user_dto};
use crate::dto::recipe_dto::RecipeDto;
use crate::models::recipe::Recipe;

pub fn to_recipe_dto(recipe: &Recipe) -> RecipeDto {
    RecipeDto {
        id: recipe.id.clone(),
        title: recipe.title.clone(),
        ingredients: recipe.ingredients.clone(),
        instructions: recipe.instructions.clone(),
        comments:recipe.comments.clone(),
        created_by: to_user_dto(&recipe.created_by)
    }
}

pub fn from_recipe_dto(recipe_dto: &RecipeDto) -> Recipe {
    Recipe {
        id: recipe_dto.id.clone(),
        title: recipe_dto.title.clone(),
        ingredients: recipe_dto.ingredients.clone(),
        instructions: recipe_dto.instructions.clone(),
        comments: recipe_dto.comments.clone(),
        created_by: from_user_dto(&recipe_dto.created_by)
    }
}
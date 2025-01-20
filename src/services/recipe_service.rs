use std::collections::HashMap;
use std::sync::RwLock;
use lazy_static::lazy_static;
use crate::models::recipe::Recipe;
use crate::dto::recipe_dto::RecipeDto;
use crate::assembler::recipe_assembler::to_recipe_dto;
use crate::models::user::User;

// Mock database setup
lazy_static! {
    static ref MOCK_DATABASE: RwLock<HashMap<i32, Recipe>> = {
        let mut m = HashMap::new();
        m.insert(1, Recipe {
            id: 1,
            title: "Example Recipe".to_string(),
            ingredients: vec!["Ingredient 1".to_string(), "Ingredient 2".to_string()],
            instructions: "Mix everything.".to_string(),
            comments: vec![],
            created_by: User {
                id: 0,
                name: "User".to_string(),
                email: "user@gmail.com".to_string(),
            },
        });
        RwLock::new(m)
    };
}

/// Get recipe by id
pub fn get_recipe_by_id(id: i32) -> Option<RecipeDto> {
    // Borrow read-only
    let db = MOCK_DATABASE.read().unwrap();

    // Read the recipe
    let recipe = db.get(&id).cloned();

    match recipe {
        Some(recipe) => Some(to_recipe_dto(&recipe)),
        None => None,
    }
}

/// Get all recipes
pub fn get_all_recipes() -> Option<Vec<RecipeDto>> {
    // Borrow read-only
    let db = MOCK_DATABASE.read().unwrap();

    // Check if there are any recipes in the database
    if db.is_empty() {
        None
    } else {
        // Map all Recipe entries to RecipeDto
        let recipe_list: Vec<RecipeDto> = db.values().map(|recipe| to_recipe_dto(recipe)).collect();
        Some(recipe_list)
    }
}

/// Add a new recipe
pub fn add_recipe(recipe: Recipe) -> bool {
    // Borrow write access
    let mut db = MOCK_DATABASE.write().unwrap();

    // Insert the recipe
    db.insert(recipe.id, recipe).is_none()
}

/// Update an existing recipe, if not found, add the recipe instead
pub fn update_recipe(updated_recipe: Recipe) -> RecipeDto {
    // Borrow read access first to check existence
    let exists = {
        let db = MOCK_DATABASE.read().unwrap();
        db.contains_key(&updated_recipe.id)
    };

    if exists {
        // Update existing recipe
        let mut db = MOCK_DATABASE.write().unwrap();
        if let Some(existing) = db.get_mut(&updated_recipe.id) {
            *existing = updated_recipe.clone();
            return to_recipe_dto(existing);
        }
    } else {
        // Add new recipe
        add_recipe(updated_recipe.clone());
    }

    to_recipe_dto(&updated_recipe)
}

/// Delete a recipe by id
pub fn delete_recipe(id: i32) -> bool {
    // Borrow write access
    let mut db = MOCK_DATABASE.write().unwrap();

    // Remove and check if it was found and removed
    db.remove(&id).is_some()
}
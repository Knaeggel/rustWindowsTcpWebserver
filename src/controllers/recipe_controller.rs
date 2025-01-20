use std::collections::HashMap;
use crate::assembler::recipe_assembler::from_recipe_dto;
use crate::dto::recipe_dto::RecipeDto;
use crate::response_builder::{create_http_bad_request_response, create_http_created_response, create_http_internal_server_error_response, create_http_not_found_response, create_http_success_response};
use crate::services::recipe_service;

pub fn get_recipe_by_id(_req_content_type: &str, query_params: Option<&str>, _req_body: &str) -> String {
    match query_params {
        Some(query_params) => {
            // Parse query parameters into key-value pairs
            let mut params_map = HashMap::new();

            for param in query_params.split('&') {
                let mut parts = param.split('=');
                if let (Some(key), Some(value)) = (parts.next(), parts.next()) {
                    params_map.insert(key.to_string(), value.to_string());
                }
            }

            if let Some(id_str) = params_map.get("id") {
                if let Ok(id) = id_str.parse::<i32>() {
                    match recipe_service::get_recipe_by_id(id) {
                        Some(recipe) => {
                            // Serialize the Recipe to JSON
                            let serialized = serde_json::to_string(&recipe).unwrap_or_else(|_| "".to_string());
                            create_http_success_response(&serialized)
                        }
                        None => {
                            create_http_not_found_response(Some("Could not find recipe"))
                        }
                    }
                } else {
                    create_http_bad_request_response("Invalid ID format")
                }
            } else {
                create_http_bad_request_response("Id missing in query parameters")
            }
        }
        None => {
            create_http_bad_request_response("Query parameters are missing")
        }
    }
}

pub fn get_all_recipes(_req_content_type: &str, query_params: Option<&str>, req_body: &str) -> String {
    // Ensure the request has no unnecessary query parameters
    if query_params.is_some() {
        return create_http_bad_request_response("This endpoint does not accept query parameters");
    }

    // Check if the content type is valid
    if req_body.is_empty() == false {
        return create_http_bad_request_response("Request body must be empty");
    }

    // Call the service to get all recipes
    match recipe_service::get_all_recipes() {
        Some(recipes) => {
            // Serialize the list of recipes to JSON
            let serialized = serde_json::to_string(&recipes).unwrap_or_else(|_| "".to_string());
            create_http_success_response(&serialized)
        }
        None => {
            create_http_not_found_response(Some("No recipes found"))
        }
    }
}


pub fn update_recipe(req_content_type: &str, query_params: Option<&str>, req_body: &str) -> String {
    match query_params {
        Some(_) => {
            create_http_bad_request_response("Can't update the recipe with query parameters")
        }
        None => {
            match req_content_type {
                "application/json" => {
                    // Deserialize the JSON request body
                    let recipe_dto: Result<RecipeDto, _> = serde_json::from_str(req_body);

                    match recipe_dto {
                        Ok(dto) => {
                            // Convert RecipeDto to Recipe model
                            let recipe = from_recipe_dto(&dto);

                            let updated_recipe = recipe_service::update_recipe(recipe);
                            let serialized = serde_json::to_string(&updated_recipe).unwrap_or_else(|_| "".to_string());
                            create_http_success_response(&serialized)

                        }
                        Err(_) => {
                            create_http_internal_server_error_response("Failed to deserialize the recipe")
                        }
                    }
                }
                _ => {
                    create_http_bad_request_response("Body must contain JSON")
                }
            }
        }
    }
}

pub fn add_recipe(req_content_type: &str, query_params: Option<&str>, req_body: &str) -> String {
    match query_params {
        Some(_) => {
            create_http_bad_request_response("Can't add the recipe with query parameters")
        }
        None => {
            match req_content_type {
                "application/json" => {
                    // Deserialize the JSON request body into a RecipeDto
                    let recipe_dto: Result<RecipeDto, _> = serde_json::from_str(req_body);

                    match recipe_dto {
                        Ok(dto) => {
                            // Convert RecipeDto to Recipe model
                            let recipe = from_recipe_dto(&dto);

                            let recipe_id = recipe.id.clone().to_string();

                            // Call the service to add the recipe
                            if recipe_service::add_recipe(recipe) {
                                create_http_created_response(&*recipe_id, "Recipe added successfully")
                            } else {
                                create_http_internal_server_error_response("Failed to add the recipe")
                            }
                        }
                        Err(_) => {
                            create_http_internal_server_error_response("Failed to deserialize the recipe")
                        }
                    }
                }
                _ => {
                    create_http_bad_request_response("Body must contain JSON")
                }
            }
        }
    }
}

pub fn delete_recipe_by_id(_req_content_type: &str, query_params: Option<&str>, _req_body: &str) -> String {
    match query_params {
        Some(query_params) => {
            // Parse query parameters into key-value pairs
            let mut params_map = HashMap::new();

            for param in query_params.split('&') {
                let mut parts = param.split('=');
                if let (Some(key), Some(value)) = (parts.next(), parts.next()) {
                    params_map.insert(key.to_string(), value.to_string());
                }
            }

            if let Some(id_str) = params_map.get("id") {
                if let Ok(id) = id_str.parse::<i32>() {
                    // Call the service to delete the recipe
                    if recipe_service::delete_recipe(id) {
                        create_http_success_response("Recipe deleted successfully")
                    } else {
                        create_http_not_found_response(Some("Recipe not found"))
                    }
                } else {
                    create_http_bad_request_response("Invalid ID format")
                }
            } else {
                create_http_bad_request_response("id missing in query parameters")
            }
        }
        None => {
            create_http_bad_request_response("Query parameters are missing")
        }
    }
}
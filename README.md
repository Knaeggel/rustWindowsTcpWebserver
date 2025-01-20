# HTTP Server Implementation in Rust

This project is a custom HTTP server implementation in Rust that supports basic routing, request handling, and dynamic responses for different HTTP methods (GET, PUT, POST, DELETE). It uses the Windows Sockets API (WinSock) to manage networking functionality, and the server dynamically dispatches requests to predefined service methods based on their path and method.

## Getting Started

### Running the Server

To start the server, execute the following command:

```bash
cargo run
```


Once the server is running, you can access it at:
http://localhost:8080

### Predefined Methods

The main provided functions are on the `RecipeController`. These functions handle requests to the `RecipeService`, which includes routes for retrieving, adding, updating, and deleting recipes.
The server can also be stopped by sending a request to the `/ShutDown` route, which triggers a shutdown of the server.

# HTTP Server Method Dispatcher

This section outlines the routes and their corresponding HTTP methods that the server supports. The dispatcher dynamically routes incoming requests to the appropriate service methods based on the path and method.

## Routes

### GET /RecipeService/GetRecipeById
- **Method**: `GET`
- **Path**: `/RecipeService/GetRecipeById?id=1`
- **Description**: Retrieves a recipe by its unique ID. The `id` should be provided as a query parameter.

### GET /RecipeService/GetAllRecipes
- **Method**: `GET`
- **Path**: `/RecipeService/GetAllRecipes`
- **Description**: Retrieves a list of all available recipes.

### PUT /RecipeService/UpdateRecipe
- **Method**: `PUT`
- **Path**: `/RecipeService/UpdateRecipe`
- **Description**: Updates an existing recipe with new information. The request body should contain a JSON object with the following structure:
    ```json
    {
        "id": 2,
        "title": "Example Recipe",
        "ingredients": ["Ingredient 1", "Ingredient 2"],
        "instructions": "Mix everything.",
        "comments": [],
        "created_by": {
            "id": 0,
            "name": "User1",
            "email": "user1@gmail.com"
        }
    }
    ```

### POST /RecipeService/AddRecipe
- **Method**: `POST`
- **Path**: `/RecipeService/AddRecipe`
- **Description**: Adds a new recipe to the system. The request body should contain a JSON object with the following structure:
    ```json
    {
        "id": 2,
        "title": "Example Recipe",
        "ingredients": ["Ingredient 1", "Ingredient 2"],
        "instructions": "Mix everything.",
        "comments": [],
        "created_by": {
            "id": 0,
            "name": "User1",
            "email": "user1@gmail.com"
        }
    }
    ```

### DELETE /RecipeService/DeleteRecipeById
- **Method**: `DELETE`
- **Path**: `/RecipeService/DeleteRecipeById?id=2`
- **Description**: Deletes a recipe based on its unique ID. The `id` should be provided as a query parameter.

---

### Example Recipe Structure

When sending or receiving a recipe in requests (for `PUT` or `POST`), the recipe should follow this JSON structure:

```json
{
    "id": 2,
    "title": "Example Recipe",
    "ingredients": ["Ingredient 1", "Ingredient 2"],
    "instructions": "Mix everything.",
    "comments": [],
    "created_by": {
        "id": 0,
        "name": "User1",
        "email": "user1@gmail.com"
    }
}


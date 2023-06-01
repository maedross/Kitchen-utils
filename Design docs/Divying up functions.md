## Frontend Functions
* Navigate to meal plan screen
    * Request meal plan recipes
    * Request grocery list
* Navigate to list of recipes screen
    * Request list of all recipes
* Navigate to new recipe screen
    * Command to create new 
    * Request ingredient data
    * Command to save recipe data
    * Command to save ingredient data
* Navigate to existing recipe screen
    * Request recipe data
    * Request ingredient data
    * Command to save recipe data
    * Command to save ingredient data
* Check box on recipe
    * Command alter inclusion of recipe in meal plan

## Backend Functions
* Load list of all recipes
   * fn all_recipes() -> Vec<(String, bool)>
* Load individual recipe
   * fn load_recipe(String) -> Recipe
* Load meal plan recipes
   * fn meal_plan() -> Vec<String>
* Load list of all ingredients
   * fn load_ingredients() -> Vec<Ingredient>
* New recipe
   * fn new_recipe(String)
* Save recipe
   * fn save_recipe(Recipe)
* Save ingredient
   * fn save_ingredient(Ingredient)
* Delete ingredient
   * fn delete_ingredient(Ingredient)
* Generate shopping list
   * fn shopping_list(Vec<usize>) -> Vec<(f64, Ingredient)>
* Add recipe to meal plan
   * fn add_to_mp(String)
* Remove recipe from meal plan
   * fn rem_from_mp(String)
## Program Functions:
* Create/Edit/View recipe:
    * View:
        * Editor with 2 sections and recipe title
        * Each section is list with ability to add item
        * Ingredients section:
            * Two aspects per item
                * Ingredient itself (as predefined)
                * Quantity
        * Directions section:
            * List of steps
        * The ability to save and exit editor
        * The ability to turn on and off editor
    * Model:
        * Fetch recipe data
        * Store recipe in recipe class
            * Recipe class has title
            * Recipe class has number total servings (1 serving = 1 meal)
            * Recipe class has 2 lists
                * Ingredients list stores couples (number and ingredient)
                * Directions list stores text (strings)
    * Controller:
        * State variable "mode" which determines editability
        * Add item to ingredients/directions list of recipe
        * Edit list item or title using some form of text input
        * Reroute to view list of recipes
        * Verify the recipe has a non-null title
        * For add ingredient section (verify type):
            * For quantity input exclusively number
            * For ingredient, choose from predefined list
                * Dropdown menu/typeassist feature
* View list of recipes
    * View:
        * List of recipes
        * Ability to add recipe
        * Ability to go back to homescreen
        * Ability to toggle recipe inclusion in meal plan
    * Model:
        * Retrieve all recipes
        * Write recipe to meal plan
    * Controller:
        * Clicking on add recipe creates new instance of recipe class and takes you to editor
        * Clicking on view recipe takes you to view recipe
        * Reroute to homepage
        * Clicking on meal plan toggle writes to meal plan changing recipe inclusion (if it's already included, remove it, and vice versa)
* Define ingredients
    * View:
        * Option on dropdown menu in recipe editor to create new ingredient
        * Fields for user to input ingredient name, and unit of measurement
    * Model:
        * Ingredient class
        * Has name
        * Has unit of measurement (ie. nothing, g, cups), which is a string
    * Controller:
        * Create/collapse ingredient view
        * Store ingredient
* Create meal plan
    * View:
        * List of recipes added to meal plan
            * Recipes have serving yield, which user can edit
        * Ability to clear meal plan
        * Ability to remove individual recipes
        * Ability to go to list of recipes
        * List of ingredients for meal plan
            * Note: might be worth checking whether the calculation should be user-prompted or by default
    * Model:
        * Retrieve meal plan recipes
        * Reset meal plan data
    * Controller:
        * Remove individual meal from plan
        * Remove all meals from plan

## Platform
* Tauri

* View:
* Model:
* Controller:
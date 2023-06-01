# /kitchen/utils
## The Problems
0. I need things to put on my resume
1. Checking what ingredients I will need when I am meal planning and making a grocery list is a pain
2. I have no idea how much my different recipes cost to prepare relative to each other or to going out to eat
3. My recipes are scattered about and a file system does not organize them as well as I would like
4. I forget when I have produce in the fridge that will go bad
5. Relatedly I may have an excess of an ingredient but am unsure what I can make that will use it

## Use Cases
* Add a recipe
    * Create new recipe file
        * Add name to recipe
    * Add ingredients to recipe + quanitity
        * Define new ingredient
            * Ingredient name
            * Ingredient price per unit(s)
            * Shelflife
            * Brand(s)
    * Add directions to recipe
    * Add tags to recipe
* Manage shopping list
    * Add constraints
        * Budget
        * Availablity
    * Select recipes
    * Edit list
        * Add/edit/remove ingredients
    * Check off items from list
        * Change necessary information (for each ingredient)
            * Name
            * Price
            * Amount
            * Expiration Date
            * Did not buy
* Inventory management
    * Update ingredient amount/shelflife
    * Be alerted when app calculates that ingredient is low
    * Be alerted when app calculates that an ingredient will expire soon
* Reference a recipe
    * Find a recipe
    * Estimate recipe price
    * Open a recipe file

# Post-Development Possibilities
* Mobile version
    * Syncing between mobile and desktop
* Recipe-sharing website
* Order stuff
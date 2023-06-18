// Might need to make this a simple frame like the App file, and have the search screen be its own component
// Otherwise will need to add a button to recipe screens to go back to search results
<template>
    <h1>Recipes</h1>
    <section>
        <NewRecipeButton/>
        <RecipeButton>Alfalfa</RecipeButton>
        <RecipeButton>Rat Tail Quiche</RecipeButton>
    </section>
</template>
    
    
    <script lang="ts">
    import NewRecipeButton from './NewRecipeButton.vue';
    import RecipeButton from './RecipeButton.vue';

    export default {
        components: {
            NewRecipeButton,
            RecipeButton,
        }
    }
    </script>
    
    <style>
        section {
            display: flex
        }
    </style>
<template>
    <div>
        <h3>Ingredients</h3>
        <ul>
            <li v-for="ingredient in ingredients" :key="ingredient">
                <button @click="$emit('ingredient-added')">{{ ingredient }}</button>
            </li>
            <div v-if="!creatingIngredient">
                <button @click="toggleNewIngredientForm">
                    + Define new ingredient
                </button>
            </div>
            <div v-else>
                <NewIngredientForm @ingredient-created="createIngredient"/>
            </div>
        </ul>
    </div>
</template>


<script lang="ts">
    import NewIngredientForm from './NewIngredientForm.vue';

    export default {
        components: {
            NewIngredientForm,
        },
        data () {
            return {
                creatingIngredient: false,
                ingredients: ["Red Mushrooms", "Soup", "Bird from window"]
            }
        },
        methods: {
            toggleNewIngredientForm() {
                this.creatingIngredient = true;
            },
            createIngredient() {
                this.creatingIngredient = false;
            }
        }
    };
</script>


<style>
</style>
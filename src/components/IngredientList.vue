<template>
    <div>
        <h3>Ingredients</h3>
        <ul>
            <li>Red Mushrooms</li>
            <li>Soup</li>
            <li>Bird from window</li>
            <div v-if="!creatingingIngredient">
                <button @click="toggleNewIngredientForm">
                    + Define new ingredient
                </button>
            </div>
            <div v-else>
                <NewIngredientForm @ingredient-added="createIngredient"/>
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
                creatingingIngredient: false,
            }
        },
        methods: {
            toggleNewIngredientForm() {
                this.creatingingIngredient = true;
            },
            createIngredient() {
                this.creatingingIngredient = false;
            }
        }
    };
</script>


<style>
</style>
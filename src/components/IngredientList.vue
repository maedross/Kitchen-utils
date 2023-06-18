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
                <NewIngredient @ingredient-added="createIngredient"/>
            </div>
        </ul>
    </div>
</template>


<script lang="ts">
    import NewIngredient from './NewIngredient.vue';

    export default {
        components: {
            NewIngredient,
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
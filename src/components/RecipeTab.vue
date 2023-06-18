<template>
    <h1>Recipe Title</h1>
    <h2>Recipe Serving Amount</h2>
    <section>
        <div>
            <ul>
                <h4>Ingredients:</h4>
                <li v-for="ingredient in ingredients" :key="ingredient">
                    {{ ingredient }}
                </li>
                <li v-if="!addingIngredient">
                    <button @click="toggleAddIngredient">+ Add ingredient</button>
                </li>
                <IngredientList @ingredient-added="toggleAddIngredient" v-else />
            </ul>
        </div>
        <div>
            <ol>
                <h4>Directions: </h4>
                <li v-for="direction in directions" :key="direction">
                    {{ direction }}    
                </li>
                <li v-if="!addingStep">
                    <button @click="toggleAddStep">+ Add step</button>
                </li>
                <form v-else @submit.prevent="onSubmit">
                    <input type="text" name="newStep" id="newStep" required >
                    <input type="submit" value="Save">
                </form>
            </ol>
        </div>
    </section>
</template>


<script lang="ts">

    import IngredientList from './IngredientList.vue';

    export default {
        components: {
            IngredientList,
        },
        data () {
            return {
                addingIngredient: false,
                addingStep: false,
                ingredients: ["Rat tail", "Eggs", "Flour", "Saliva (cow)"],
                directions: ["Break the eggs", "Enjoy!"]
            }
        },
        methods: {
            toggleAddIngredient() {
                this.addingIngredient = !this.addingIngredient
            },
            toggleAddStep() {
                this.addingStep = true
            },
            onSubmit() {
                this.addingStep = false
            }
        }
    };
</script>


<style>
</style>
<template>
    <h1>Recipe Title</h1>
    <h2>Recipe Serving Amount</h2>
    <section>
        <div>
            <ul>
                Ingredients
                <li>Rat tail</li>
                <li>Eggs</li>
                <li>Flour</li>
                <li>Saliva (cow)</li>
                <li v-if="!addingIngredient">
                    <button @click="toggleAddIngredient">+ Add ingredient</button>
                </li>
                <IngredientList v-else />
            </ul>
        </div>
        <div>
            <ol>
                Directions: 
                <li>Break the eggs</li>
                <li>Enjoy!</li>
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
                addingStep: false
            }
        },
        methods: {
            toggleAddIngredient() {
                this.addingIngredient = true
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
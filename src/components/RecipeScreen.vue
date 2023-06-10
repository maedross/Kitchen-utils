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
                <li>
                    <button>+ Add ingredient</button>
                </li>
            </ul>
        </div>
        <div>
            <ol>
                Directions: 
                <li>Break the eggs</li>
                <li>Enjoy!</li>
                <li>
                    <button>+ Add step</button>
                </li>
            </ol>
        </div>
    </section>
</template>


<script lang="ts">

    import IngredientList from './IngredientList.vue';

    export default {

    };
</script>


<style>
</style>
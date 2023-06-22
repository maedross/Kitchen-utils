<template>
<h1>Meal Plan</h1>
<h2>{{ backend_message }}</h2>
<section>
    <ol>
        <h2>Selected Recipes:</h2>
        <li v-for="meal in meals" key="meal">
            {{ meal }}
        </li>
    </ol>
    <ol>
        <h2>Shopping List:</h2>
        <li v-for="ingredient in ingredients" key="ingredient">
            {{ ingredient }}
        </li>
    </ol>
</section>
</template>


<script lang="ts">
import { invoke } from '@tauri-apps/api/tauri'

export default {
    components: {

    },
    data () {
        return {
            meals: ["Alfalfa (raw)", "Rat Tail Quiche", "Gruel"],
            ingredients: ["Alfalfa seeds", "Rat tails", "Sludge"],
            backend_message: invoke('my_custom_command').then()
        }
    }
}
</script>

<style>
    section {
        display: flex
    }
</style>
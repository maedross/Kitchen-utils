<template>
    <form @submit.prevent="onSubmit">
        <div>
            <label for="name">Name:</label>
            <input type="text" name="name" id="name" required>
        </div>
        <div>
            <label for="measurement">Unit of measurement:</label>
            <input type="text" name="measurement" id="measurement" required>
        </div>
        <div>
            <input type="submit" value="Save">
        </div>
    </form>
</template>


<script lang="ts">
    export default {
        methods: {
            onSubmit() {
                // Will we emit an event, pass something to the backend, or both?
                this.$emit("ingredient-added");
            }
        }
    };
</script>


<style>
</style>
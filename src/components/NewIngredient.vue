<template>
    <form>
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
        
    };
</script>


<style>
</style>
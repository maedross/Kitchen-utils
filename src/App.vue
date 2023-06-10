<template>
  <div>
    <button 
      v-for="tab in tabs" 
      :key="tab" 
      :class="['tab-button', { active: currentTab === tab }]" 
      @click="currentTab = tab"
    >
      {{ tab }}
    </button>
  </div>
  <component :is="currentTab" class="tab"></component>
</template>

<script lang="ts">
import MealPlanScreen from './components/MealPlanScreen.vue';
import RecipeListScreen from './components/RecipeListScreen.vue';

export default {
  components: {
    MealPlanScreen,
    RecipeListScreen
  },
  data() {
    return {
      currentTab: "MealPlanScreen",
      tabs: ["MealPlanScreen", "RecipeListScreen"]
    }
  }
}
</script>

<style scoped>

</style>

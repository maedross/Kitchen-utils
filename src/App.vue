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
import RecipeControlScreen from './components/RecipeControlScreen.vue';


export default {
  components: {
    MealPlanScreen,
    RecipeControlScreen
  },
  data() {
    return {
      currentTab: "MealPlanScreen",
      tabs: ["MealPlanScreen", "RecipeControlScreen"]
    }
  }
}
</script>

<style scoped>

</style>

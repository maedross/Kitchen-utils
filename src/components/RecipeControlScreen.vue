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
import RecipeListTab from './RecipeListTab.vue';
import RecipeTab from './RecipeTab.vue';

export default {
  components: {
    RecipeListTab,
    RecipeTab
  },
  data() {
    return {
      currentTab: "MealPlanScreen",
      tabs: ["RecipeListTab", "RecipeTab"]
    }
  }
}
</script>

<style scoped>

</style>

<script setup lang="ts">
import SearchBoxes from './SearchBoxes.vue';
import SearchConditions from './SearchConditions.vue';
</script>

<template>
  <div class="container">
    <SearchBoxes />
    <SearchConditions />
  </div>
</template>

<style scoped>
.container {
  display: flex;
  flex-direction: column;
  align-items: flex-start;
  gap: 61px;
  align-self: stretch;
}
</style>

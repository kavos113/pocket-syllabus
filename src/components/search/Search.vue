<script setup lang="ts">
import FetchButton from './FetchButton.vue';
import SearchField from './SearchField.vue';
</script>

<template>
  <div class="search-wrapper">
    <FetchButton />
    <SearchField />
  </div>
</template>

<style scoped>
.search-wrapper {
  display: flex;
  flex-direction: column;
  align-items: flex-start;
  gap: 77px;
  flex-shrink: 0;
}
</style>

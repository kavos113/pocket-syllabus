<script setup lang="ts">
import FetchButton from './FetchButton.vue';
import SearchField from './SearchField.vue';
</script>

<template>
  <div class="container">
    <FetchButton />
    <SearchField />
  </div>
</template>

<style scoped>
.container {
  display: flex;
  flex-direction: column;
  align-items: flex-start;
  gap: 77px;
  flex-shrink: 0;
}
</style>

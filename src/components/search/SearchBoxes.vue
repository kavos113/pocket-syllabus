<script setup lang="ts">
import ComboBox from '../common/ComboBox.vue';
import SearchBox from '../common/SearchBox.vue';
</script>

<template>
  <div class="container">
    <ComboBox />
    <ComboBox />
    <ComboBox />
    <SearchBox />
    <SearchBox />
  </div>
</template>

<style scoped>
.container {
  display: flex;
  width: 100%;
  flex-direction: column;
  align-items: flex-start;
  gap: 24px;
}
</style>

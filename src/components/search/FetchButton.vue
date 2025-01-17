<script setup lang="ts">
import SimpleButton from '../common/SimpleButton.vue';
</script>

<template>
  <div class="container">
    <SimpleButton text="Fetch" />
    <SimpleButton text="Fetch-Test" />
  </div>
</template>

<style scoped>
.container {
  display: flex;
  align-items: center;
  flex-direction: row;
  gap: 28px;
}
</style>

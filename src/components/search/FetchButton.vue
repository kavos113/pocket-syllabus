<script setup lang="ts">
import SimpleButton from '../common/SimpleButton.vue';
</script>

<template>
  <div class="button-wrapper">
    <SimpleButton text="Fetch" />
    <SimpleButton text="Fetch-Test" />
  </div>
</template>

<style scoped>
.button-wrapper {
  display: flex;
  align-items: center;
  flex-direction: row;
  gap: 28px;
}
</style>

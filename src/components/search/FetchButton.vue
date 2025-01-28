<script setup lang="ts">
import SimpleButton from '../common/SimpleButton.vue';
import { invoke } from '@tauri-apps/api/core';

async function greeting() {
  console.log('Fetching...');
  await invoke('fetch_test');
}
</script>

<template>
  <div class="button-wrapper">
    <SimpleButton text="Fetch" />
    <SimpleButton
      text="Fetch-Test"
      @click="greeting"
    />
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

<script setup lang="ts">
import SimpleButton from '../common/SimpleButton.vue';
import { invoke } from '@tauri-apps/api/core';
import { ref } from 'vue';
import { listen } from '@tauri-apps/api/event';

async function greeting() {
  console.log('Fetching...');
  await invoke('fetch_test');
}

async function fetch() {
  console.log('Fetching...');
  await invoke('fetch');
}

const fetchStatus = ref('Not fetched');

listen<string>('fetch_status', (event) => {
  fetchStatus.value = event.payload;
});
</script>

<template>
  <div>
    <div class="button-wrapper">
      <SimpleButton
        text="Fetch"
        @click="fetch"
      />
      <SimpleButton
        text="Fetch-Test"
        @click="greeting"
      />
    </div>
    <div>
      <p>{{ fetchStatus }}</p>
    </div>
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

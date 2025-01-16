<script setup lang="ts">
import { ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import './styles/style.css';
import Title from './components/Title.vue';
import Content from './components/Content.vue';

const greetMsg = ref('');
const name = ref('');

async function greet() {
  // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
  greetMsg.value = await invoke('greet', { name: name.value });
}
</script>

<template>
  <main class="container">
    <div class="main">
      <Title />
      <Content />
    </div>
  </main>
</template>

<style scoped>
.container {
  width: 100%;
  height: 100vh;
  background: var(--grayborder);
}

.main {
  display: flex;
  width: calc(100% - 40px);
  height: calc(100% - 35px);
  padding: 25px 20px 10px 20px;
  flex-direction: column;
  align-items: center;
  gap: 10px;
  flex-shrink: 0;
}
</style>

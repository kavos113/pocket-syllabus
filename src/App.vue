<script setup lang="ts">
import './styles/style.css';
import Title from './components/Title.vue';
import Content from './components/Content.vue';
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

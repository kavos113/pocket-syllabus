<script setup lang="ts">
import { ref } from 'vue';

const text = ref<string>('Hello, Vue 3!');
</script>

<template>
  <div class="container">
    <p class="text">{{ text }}</p>
  </div>
</template>

<style scoped>
.container {
  width: 270px;
  height: 28px;
  flex-shrink: 0;
  border-radius: 12px;
  background: var(--white);
}

.text {
  display: flex;
  width: 100%;
  height: 28px;
  flex-direction: column;
  justify-content: center;
  flex-shrink: 0;
  color: var(--allblack, #000);
  font-family: 'Rounded Mplus 1c Medium', var(--font), sans-serif;
  font-size: 14px;
  font-style: normal;
  font-weight: 400;
  line-height: normal;
  margin: 0;
  padding-left: 12px;
}
</style>

<script setup lang="ts">
const props = defineProps<{
  text: string;
}>();
</script>

<template>
  <div class="button">
    <p class="text">{{ props.text }}</p>
  </div>
</template>

<style scoped>
.text {
  display: flex;
  width: 110px;
  height: 39px;
  flex-direction: column;
  justify-content: center;
  flex-shrink: 0;
  color: var(--3black);
  text-align: center;
  font-family: 'Rounded Mplus 1c Medium', var(--font), sans-serif;
  font-size: 20px;
  font-style: normal;
  font-weight: 400;
  line-height: normal;
  margin: 0;
}

.button {
  border-radius: 8px;
  background: var(--background);
  cursor: pointer;
  border: 1px solid var(--background);
}

.button:hover {
  border: 1px solid var(--stmain);
}
</style>

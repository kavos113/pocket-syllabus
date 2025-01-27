<script setup lang="ts">
import { defineProps } from 'vue';

const props = defineProps<{
  checkboxId: string;
  content: string;
}>();
</script>
<template>
  <div class="check-box-container">
    <input
      type="checkbox"
      :id="props.checkboxId"
      class="check-box"
    />
    <label
      :for="props.checkboxId"
      class="label"
      >{{ content }}</label
    >
  </div>
</template>

<style scoped>
.check-box {
  width: 16px;
  height: 16px;
  flex-shrink: 0;
  border-radius: 2px;
  background: #fff;
}

.label {
  color: #000;
  font-family: 'Rounded Mplus 1c', var(--font), sans-serif;
  font-size: 14px;
  font-style: normal;
  font-weight: 400;
  line-height: normal;
}

.check-box-container {
  display: flex;
  width: 90px;
  height: 21px;
  justify-content: left;
  align-items: center;
  gap: 2px;
}
</style>

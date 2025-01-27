<script setup lang="ts">
import CheckBox from '../common/CheckBox.vue';

const props = defineProps<{
  checkboxId: string;
  contents: string[];
}>();
</script>

<template>
  <div class="check-boxes-container">
    <CheckBox
      v-for="index in props.contents.length"
      :key="index"
      :checkbox-id="props.checkboxId + '1'"
      :content="props.contents[index - 1]"
    />
  </div>
</template>

<style scoped></style>

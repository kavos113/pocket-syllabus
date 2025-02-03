<script setup lang="ts">
const props = defineProps<{
  placeholder: string;
}>();

const textModel = defineModel<string>();
</script>

<template>
  <div class="search-box-container">
    <input
      class="text"
      type="text"
      v-model="textModel"
      :placeholder="props.placeholder"
    />
  </div>
</template>

<style scoped>
.search-box-container {
  width: 352px;
  height: 28px;
  flex-shrink: 0;
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
  border: none;
}

.text:focus {
  outline: none;
}
</style>

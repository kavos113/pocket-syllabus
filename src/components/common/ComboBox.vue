<script setup lang="ts">
import ComboBoxMenu from './ComboBoxMenu.vue';

const props = defineProps<{
  items: object;
}>();
</script>

<template>
  <div class="combobox-box-container">
    <div class="comboMenu">
      <ComboBoxMenu :items="props.items" />
    </div>
  </div>
</template>

<style scoped>
.combobox-box-container {
  width: 270px;
  height: 28px;
  flex-shrink: 0;
  border-radius: 12px;
  background: var(--white);
  position: relative;
}

.comboMenu {
  position: absolute;
  top: 0;
  left: 0;
  width: 270px;
  border-radius: 12px;
  background: var(--white);
}

.comboMenu > .menu {
  background: var(--ablack);
  box-shadow: none;
  border-radius: 12px;
}
</style>

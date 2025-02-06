<script setup lang="ts">
import ComboBoxMenu from './ComboBoxMenu.vue';
import { ref } from 'vue';
import { Menu } from '../../scripts/consts.ts';

const props = defineProps<{
  items: Menu;
}>();

const emits = defineEmits<{
  (event: 'selectItem', items: string[]): void;
}>();

const items = ref<string[]>([]);

const selectItem = (key: string) => {
  let flag = false;
  items.value.forEach((item) => {
    if (item === key) {
      items.value = items.value.filter((value) => value !== key);
      flag = true;
    }
  });
  if (!flag) {
    items.value.push(key);
  }
  console.log(items.value);
  emits('selectItem', items.value);
};
</script>

<template>
  <div class="combobox-box-container">
    <div class="comboMenu">
      <ComboBoxMenu
        :items="props.items"
        @click-menu-item="selectItem"
      />
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

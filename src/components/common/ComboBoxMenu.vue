<script setup lang="ts">
import { ref } from 'vue';

const props = defineProps<{
  items: object;
}>();

const isObject = (value: any) => {
  return value && typeof value === 'object' && value.constructor === Object;
};

const isArray = (value: any) => {
  return value && typeof value === 'object' && value.constructor === Array;
};

const openItems = ref<Record<string, boolean>>({});

const openSubMenu = (key: string) => {
  openItems.value[key] = true;
};

const closeSubMenu = (key: string) => {
  openItems.value[key] = false;
};

const isOpen = (key: string) => {
  return openItems.value[key];
};
</script>

<template>
  <ul class="menu">
    <li
      v-for="(item, key) in props.items"
      :key="key"
      class="menuItem"
      @mouseenter="openSubMenu(key)"
      @mouseleave="closeSubMenu(key)"
    >
      <div class="menuTitle">
        <span class="menuText">{{ key }}</span>
        <span
          v-if="isObject(items)"
          class="arrow"
          >▶</span
        >
      </div>
      <template v-if="isOpen(key)">
        <ComboBoxMenu
          :items="item"
          v-if="isObject(item)"
          class="subMenu"
        />
        <ul
          v-else-if="isArray(item)"
          class="subMenu"
        >
          <li
            v-for="subItem in item"
            :key="subItem"
            class="menuItem"
          >
            <span class="menuText">{{ subItem }}</span>
          </li>
        </ul>
      </template>
    </li>
  </ul>
</template>

<style scoped>
.menu {
  list-style-type: none;
  padding: 0;
  margin: 0;
  font-size: 12px;
  background: var(--white);
  box-shadow: 0 2px 4px var(--stmain);
}

.menuItem {
  position: relative;
  border-bottom: 1px solid #ddd;
  padding: 0.5em 1em;
  cursor: pointer;
  background: var(--white);
}

.menuItem:last-child {
  border-bottom: none;
}

.menuItem:hover {
  background: #f0f0f0;
}

.subMenu {
  list-style-type: none;
  position: absolute;
  left: 100%;
  top: 0;
  width: 100%;
  margin: 0;
  padding: 0;
  box-shadow: 0 2px 4px var(--stmain);
  z-index: 100;
}

.arrow {
  position: absolute;
  right: 10px;
}

.menuText {
  color: var(--allblack, #000);
  font-family: 'Rounded Mplus 1c Medium', var(--font), sans-serif;
  font-size: 14px;
  font-style: normal;
  font-weight: 400;
  line-height: normal;
}
</style>

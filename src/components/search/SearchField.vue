<script setup lang="ts">
import SearchBoxes from './SearchBoxes.vue';
import SearchConditions from './SearchConditions.vue';
import { SearchComboBox, SearchConditionsType } from './Search.vue';

const emit = defineEmits<{
  (event: 'clickMenuItem', key: SearchConditionsType, items: string[]): void;
}>();

const onSearchBoxSelect = (key: SearchComboBox, items: string[]) => {
  emit('clickMenuItem', key, items);
};

const onChangeSearchBox = (title: string, lecturer: string) => {
  emit('clickMenuItem', 'title', [title]);
  emit('clickMenuItem', 'lecturer', [lecturer]);
};

const onChangeSearchConditions = (
  type: SearchConditionsType,
  items: string[],
) => {
  emit('clickMenuItem', type, items);
};
</script>

<template>
  <div class="search-container">
    <SearchBoxes
      @click-menu-item="onSearchBoxSelect"
      @change-search-box="onChangeSearchBox"
    />
    <SearchConditions @check-item="onChangeSearchConditions" />
  </div>
</template>

<style scoped>
.search-container {
  display: flex;
  flex-direction: column;
  align-items: flex-start;
  gap: 61px;
  align-self: stretch;
}
</style>

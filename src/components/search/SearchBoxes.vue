<script setup lang="ts">
import ComboBox from '../common/ComboBox.vue';
import SearchBox from '../common/SearchBox.vue';
import { DEPARTMENTS, UNIVERCITIES, YEARS } from '../../scripts/consts.ts';
import { SearchComboBox } from './Search.vue';

const emit = defineEmits<{
  (event: 'clickMenuItem', key: SearchComboBox, items: string[]): void;
}>();

const onUniversitySelect = (items: string[]) => {
  emit('clickMenuItem', 'university', items);
};

const onDepartmentSelect = (items: string[]) => {
  emit('clickMenuItem', 'department', items);
};

const onYearSelect = (items: string[]) => {
  emit('clickMenuItem', 'year', items);
};
</script>

<template>
  <div class="search-box-wrapper">
    <ComboBox
      :items="UNIVERCITIES"
      @select-item="onUniversitySelect"
    />
    <ComboBox
      :items="DEPARTMENTS"
      @select-item="onDepartmentSelect"
    />
    <ComboBox
      :items="YEARS"
      @select-item="onYearSelect"
    />
    <SearchBox />
    <SearchBox />
  </div>
</template>

<style scoped>
.search-box-wrapper {
  display: flex;
  width: 100%;
  flex-direction: column;
  align-items: flex-start;
  gap: 24px;
}
</style>

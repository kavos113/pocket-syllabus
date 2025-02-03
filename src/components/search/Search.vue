<script setup lang="ts">
import FetchButton from './FetchButton.vue';
import SearchField from './SearchField.vue';
import { ref } from 'vue';
import SimpleButton from '../common/SimpleButton.vue';

export type SearchComboBox = 'university' | 'department' | 'year';
export type SearchSearchBox = 'title' | 'lecturer';
export type SearchCheckBox = 'grade' | 'quater';
export type SearchTimetable = 'timetable';
export type SearchConditionsType =
  | SearchComboBox
  | SearchSearchBox
  | SearchCheckBox
  | SearchTimetable;

const condition = ref<Record<SearchConditionsType, string[]>>({
  university: [],
  department: [],
  year: [],
  title: [],
  lecturer: [],
  grade: [],
  quater: [],
  timetable: [],
});

const onSearchConditionChange = (
  key: SearchConditionsType,
  items: string[],
) => {
  condition.value[key] = items;
  text.value = `university: ${condition.value.university}\ndepartment: ${condition.value.department}\nyear: ${condition.value.year}\ntitle: ${condition.value.title}\nlecturer: ${condition.value.lecturer}\ngrade: ${condition.value.grade}\nquater: ${condition.value.quater}\ntimetable: ${condition.value.timetable}`;
};

const text = ref<string>('Total: ');
</script>

<template>
  <div class="search-wrapper">
    <FetchButton />
    <SearchField @click-menu-item="onSearchConditionChange" />
    <div>
      <SimpleButton text="Search" />
      <p>{{ text }}</p>
    </div>
  </div>
</template>

<style scoped>
.search-wrapper {
  display: flex;
  flex-direction: column;
  align-items: flex-start;
  gap: 77px;
  flex-shrink: 0;
}
</style>

<script setup lang="ts">
import FetchButton from './FetchButton.vue';
import SearchField from './SearchField.vue';
import { ref } from 'vue';
import SimpleButton from '../common/SimpleButton.vue';
import { Day, Period } from '../../scripts/consts.ts';
import { invoke } from '@tauri-apps/api/core';

export type SearchComboBox = 'university' | 'department' | 'year';
export type SearchSearchBox = 'title' | 'lecturer';
export type SearchCheckBox = 'grade' | 'quarter';
export type SearchConditionsType =
  | SearchComboBox
  | SearchSearchBox
  | SearchCheckBox;
export type SearchTimetableQuery = {
  day: Day;
  period: Period;
};

interface SearchQuery {
  university: string[];
  department: string[];
  year: string[];
  title: string[];
  lecturer: string[];
  grade: string[];
  quarter: string[];
  timetable: SearchTimetableQuery[];
}

const condition = ref<SearchQuery>({
  university: [],
  department: [],
  year: [],
  title: [],
  lecturer: [],
  grade: [],
  quarter: [],
  timetable: [],
});

const onSearchConditionChange = (
  key: SearchConditionsType,
  items: string[],
) => {
  condition.value[key] = items;
  text.value = `
  university: ${condition.value.university}\n
  department: ${condition.value.department}\n
  year: ${condition.value.year}\n
  title: ${condition.value.title}\n
  lecturer: ${condition.value.lecturer}\n
  grade: ${condition.value.grade}\n
  quarter: ${condition.value.quarter}\n
  timetable: ${JSON.stringify(condition.value.timetable)}`;
};

const onTimeTable = (items: SearchTimetableQuery[]) => {
  condition.value.timetable = items;
  text.value = `
  university: ${condition.value.university}\n
  department: ${condition.value.department}\n
  year: ${condition.value.year}\n
  title: ${condition.value.title}\n
  lecturer: ${condition.value.lecturer}\n
  grade: ${condition.value.grade}\n
  quarter: ${condition.value.quarter}\n
  timetable: ${JSON.stringify(condition.value.timetable)}`;
};

const onSearch = async () => {
  invoke('search_courses', { searchQuery: condition.value }).then(() => {
    console.log('Searched');
  });
};

const text = ref<string>('Total: ');
</script>

<template>
  <div class="search-wrapper">
    <FetchButton />
    <SearchField
      @click-menu-item="onSearchConditionChange"
      @timetable="onTimeTable"
    />
    <div>
      <SimpleButton
        text="Search"
        @click="onSearch"
      />
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

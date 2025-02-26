<script setup lang="ts">
import FetchButton from './FetchButton.vue';
import SearchField from './SearchField.vue';
import { ref } from 'vue';
import SimpleButton from '../common/SimpleButton.vue';
import {
  Day,
  DayQuery,
  Period,
  PeriodQuery,
  SemesterQuery,
  toDayQuery,
  toPeriodQuery,
} from '../../scripts/consts.ts';
import { invoke } from '@tauri-apps/api/core';
import { CourseListItem } from '../../scripts/course.ts';

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

export type SearchTimetableForQuery = {
  day: DayQuery;
  period: PeriodQuery;
};

type Grade = '100' | '200' | '300' | '400' | '500' | '600';

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

interface SearchForQuery {
  university: string[];
  department: string[];
  year: string[];
  title: string[];
  lecturer: string[];
  grade: Grade[];
  quarter: SemesterQuery[];
  timetable: SearchTimetableForQuery[];
}

const emits = defineEmits<{
  (event: 'search', results: CourseListItem[]): void;
  (event: 'back'): void;
}>();

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
  university.value = condition.value.university;
  department.value = condition.value.department;
  year.value = condition.value.year;
};

const onTimeTable = (items: SearchTimetableQuery[]) => {
  condition.value.timetable = items;
  university.value = condition.value.university;
  department.value = condition.value.department;
  year.value = condition.value.year;
};

const onSearch = async () => {
  const searchForQuery: SearchForQuery = {
    university: condition.value.university,
    department: condition.value.department,
    year: condition.value.year,
    title: condition.value.title,
    lecturer: condition.value.lecturer,
    grade: [],
    quarter: [],
    timetable: [],
  };

  condition.value.quarter.forEach((item) => {
    if (item === '1Q') {
      searchForQuery.quarter.push('First');
    } else if (item === '2Q') {
      searchForQuery.quarter.push('Second');
    } else if (item === '3Q') {
      searchForQuery.quarter.push('Third');
    } else if (item === '4Q') {
      searchForQuery.quarter.push('Fourth');
    }
  });

  condition.value.timetable.forEach((item) => {
    searchForQuery.timetable.push({
      day: toDayQuery(item.day),
      period: toPeriodQuery(item.period),
    });
  });

  condition.value.grade.forEach((item) => {
    if (item === '学士1年') {
      searchForQuery.grade.push('100');
    } else if (item === '学士2年') {
      searchForQuery.grade.push('200');
    } else if (item === '学士3年') {
      searchForQuery.grade.push('300');
    } else if (item === '修士1年') {
      searchForQuery.grade.push('400');
    } else if (item === '修士2年') {
      searchForQuery.grade.push('500');
    } else if (item === '博士課程') {
      searchForQuery.grade.push('600');
    }
  });

  invoke('search_courses', { searchQuery: searchForQuery }).then((results) => {
    emits('search', results as CourseListItem[]);
    emits('back');
  });
};

const department = ref<string[]>([]);
const university = ref<string[]>([]);
const year = ref<string[]>([]);
</script>

<template>
  <div class="search-wrapper">
    <FetchButton class="fetch" />
    <SearchField
      @click-menu-item="onSearchConditionChange"
      @timetable="onTimeTable"
    />
    <div>
      <SimpleButton
        text="Search"
        @click="onSearch"
        class="button"
      />
      <p>大学: {{ university.join(', ') }}</p>
      <p>開講: {{ department.join(', ') }}</p>
      <p>年度: {{ year.join(', ') }}</p>
      <SimpleButton
        text="戻る"
        @click="emits('back')"
        class="back"
      />
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

.button {
  width: 110px;
}

.back {
  display: none;
}

@media (max-width: 600px) {
  .fetch {
    display: none;
  }

  .back {
    display: block;
  }

  .search-wrapper {
    background-color: var(--grayborder);
    box-shadow: 0 0 20px 0 rgba(0, 0, 0, 1);
    padding: 10px;
  }
}
</style>

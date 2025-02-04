<script setup lang="ts">
import ListTable from './list/ListTable.vue';
import Search from './search/Search.vue';
import { ref } from 'vue';
import { CourseListItem } from '../scripts/course.ts';

const listItems = ref<CourseListItem[]>([]);

const onSearch = (results: CourseListItem[]) => {
  listItems.value = results;
};

const isSortTitleAsc = ref<boolean>(true);
const isSortCodeAsc = ref<boolean>(true);
const isSortLecturerAsc = ref<boolean>(true);
const isSortDepartmentAsc = ref<boolean>(true);
const isSortTimetableAsc = ref<boolean>(true);

const onSortTitle = () => {
  listItems.value.sort((a, b) => {
    if (a.title < b.title) return isSortTitleAsc.value ? -1 : 1;
    if (a.title > b.title) return isSortTitleAsc.value ? 1 : -1;
    return 0;
  });
  isSortTitleAsc.value = !isSortTitleAsc.value;
};

const onSortCode = () => {
  listItems.value.sort((a, b) => {
    if (a.code < b.code) return isSortCodeAsc.value ? -1 : 1;
    if (a.code > b.code) return isSortCodeAsc.value ? 1 : -1;
    return 0;
  });
  isSortCodeAsc.value = !isSortCodeAsc.value;
};

const onSortLecturer = () => {
  listItems.value.sort((a, b) => {
    if (a.lecturer < b.lecturer) return isSortLecturerAsc.value ? -1 : 1;
    if (a.lecturer > b.lecturer) return isSortLecturerAsc.value ? 1 : -1;
    return 0;
  });
  isSortLecturerAsc.value = !isSortLecturerAsc.value;
};

const onSortDepartment = () => {
  listItems.value.sort((a, b) => {
    if (a.department < b.department) return isSortDepartmentAsc.value ? -1 : 1;
    if (a.department > b.department) return isSortDepartmentAsc.value ? 1 : -1;
    return 0;
  });
  isSortDepartmentAsc.value = !isSortDepartmentAsc.value;
};

const onSortTimetable = () => {
  listItems.value.sort((a, b) => {
    if (a.timetable < b.timetable) return isSortTimetableAsc.value ? -1 : 1;
    if (a.timetable > b.timetable) return isSortTimetableAsc.value ? 1 : -1;
    return 0;
  });
  isSortTimetableAsc.value = !isSortTimetableAsc.value;
};

const onSort = (key: string) => {
  console.log(key);
  switch (key) {
    case 'title':
      onSortTitle();
      break;
    case 'code':
      onSortCode();
      break;
    case 'lecturer':
      onSortLecturer();
      break;
    case 'department':
      onSortDepartment();
      break;
    case 'timetable':
      onSortTimetable();
      break;
  }
};
</script>

<template>
  <div class="content-container">
    <Search
      class="search"
      @search="onSearch"
    />
    <ListTable
      class="table"
      :items="listItems"
      @sort="onSort"
    />
  </div>
</template>

<style scoped>
.content-container {
  display: grid;
  width: 100%;
  height: calc(100% - 74px);
  align-items: center;
  gap: 20px;
  grid-template-columns: repeat(3, 1fr);
  grid-template-rows: 1fr;
}

.table {
  grid-column: 2 / 4;
  grid-row: 1 / 2;
}

.search {
  grid-column: 1 / 2;
  grid-row: 1 / 2;
}
</style>

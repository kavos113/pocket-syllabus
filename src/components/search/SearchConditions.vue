<script setup lang="ts">
import CheckBoxes from './CheckBoxes.vue';
import Timetable from './Timetable.vue';
import { GRADES, QUARTERS } from '../../scripts/consts.ts';
import { SearchCheckBox, SearchTimetableQuery } from './Search.vue';

const emits = defineEmits<{
  (event: 'checkItem', type: SearchCheckBox, item: string[]): void;
  (event: 'timetable', items: SearchTimetableQuery[]): void;
}>();

const onCheckGrade = (items: string[]) => {
  emits('checkItem', 'grade', items);
};

const onCheckQuater = (items: string[]) => {
  emits('checkItem', 'quarter', items);
};

const onCheckTimetable = (items: SearchTimetableQuery[]) => {
  emits('timetable', items);
};
</script>

<template>
  <div class="search-conditions-container">
    <CheckBoxes
      checkbox-id="grade"
      :contents="GRADES"
      @check-item="onCheckGrade"
    />
    <CheckBoxes
      checkbox-id="quater"
      :contents="QUARTERS"
      @check-item="onCheckQuater"
    />
    <Timetable @check-item="onCheckTimetable" />
  </div>
</template>

<style scoped>
.search-conditions-container {
  display: flex;
  align-items: flex-start;
  gap: 32px;
  align-self: stretch;
}
</style>

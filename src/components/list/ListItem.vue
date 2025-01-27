<script setup lang="ts">
import { CourseListItem } from '../../scripts/course.ts';

const props = defineProps<{
  item: CourseListItem;
}>();
</script>

<template>
  <div class="item-wrapper">
    <div class="item university">
      <p class="text">{{ props.item.university }}</p>
    </div>
    <div class="item code">
      <p class="text">{{ props.item.code }}</p>
    </div>
    <div class="item name">
      <p class="text">{{ props.item.name }}</p>
    </div>
    <div class="item lecturer">
      <p class="text">{{ props.item.lecturer }}</p>
    </div>
    <div class="item timetable">
      <p class="text">{{ props.item.timeTable }}</p>
    </div>
    <div class="item semester">
      <p class="text">{{ props.item.semester }}</p>
    </div>
    <div class="item department">
      <p class="text">{{ props.item.department }}</p>
    </div>
    <div class="item credit">
      <p class="text">{{ props.item.credit }}</p>
    </div>
  </div>
</template>

<style scoped>
.item {
  border: 1px solid var(--ablack);
  height: 18px;
  display: flex;
  align-items: center;
}

.text {
  color: var(--3black);
  font-family: 'Rounded Mplus 1c', var(--font), sans-serif;
  font-size: 12px;
  font-style: normal;
  font-weight: 400;
  line-height: normal;
  margin: auto 0 auto 0;
  padding-left: 5px;
}

.item-wrapper {
  display: grid;
  grid-template-columns: repeat(24, 1fr);
  grid-template-rows: 1fr;
}

.university {
  grid-column: 1 / 3;
  grid-row: 1 / 2;
}

.code {
  grid-column: 3 / 5;
  grid-row: 1 / 2;
}

.name {
  grid-column: 5 / 12;
  grid-row: 1 / 2;
}

.lecturer {
  grid-column: 12 / 15;
  grid-row: 1 / 2;
}

.timetable {
  grid-column: 15 / 18;
  grid-row: 1 / 2;
}

.semester {
  grid-column: 18 / 20;
  grid-row: 1 / 2;
}

.department {
  grid-column: 20 / 23;
  grid-row: 1 / 2;
}

.credit {
  grid-column: 23 / 25;
  grid-row: 1 / 2;
}
</style>

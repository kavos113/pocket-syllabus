<script setup lang="ts">
import { CourseListItem } from '../../scripts/course.ts';
import ListHeaderItem from './ListHeaderItem.vue';
import ListItem from './ListItem.vue';

const props = defineProps<{
  items: CourseListItem[];
}>();
</script>

<template>
  <div class="table">
    <ListHeaderItem />
    <div
      v-for="item in props.items"
      :key="item.id"
    >
      <ListItem :item="item" />
    </div>
  </div>
</template>

<style scoped>
.table {
  overflow-y: auto;
  height: 100%;
}
</style>

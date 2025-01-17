<script setup lang="ts">
import { ref } from 'vue';
import { CourseListItem, getSampleItems } from '../../scripts/course.ts';
import ListHeaderItem from './ListHeaderItem.vue';
import ListItem from './ListItem.vue';

const items = ref<CourseListItem[]>(getSampleItems(100));
</script>

<template>
  <div class="table">
    <ListHeaderItem />
    <div
      v-for="item in items"
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

<script setup lang="ts">
import CheckBoxes from './CheckBoxes.vue';
import Timetable from './Timetable.vue';
import { GRADES, QUATERS } from '../../scripts/consts.ts';
</script>

<template>
  <div class="search-conditions-container">
    <CheckBoxes
      checkbox-id="grade"
      :contents="GRADES"
    />
    <CheckBoxes
      checkbox-id="quater"
      :contents="QUATERS"
    />
    <Timetable />
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

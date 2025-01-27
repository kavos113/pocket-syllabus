<script setup lang="ts">
import { ref } from 'vue';
import { DAYS, PERIODS } from '../../scripts/consts.ts';

const days = ref<string[]>(DAYS);
const periods = ref<string[]>(PERIODS);
</script>

<template>
  <div class="timetable-container">
    <table class="table">
      <thead>
        <tr>
          <th class="left"></th>
          <th
            v-for="day in days"
            :key="day"
            class="mainContent"
          >
            {{ day }}
          </th>
        </tr>
      </thead>
      <tbody>
        <tr
          v-for="period in periods"
          :key="period"
        >
          <th class="left">{{ period }}</th>
          <td
            v-for="day in days"
            :key="day"
            class="mainContent"
          ></td>
        </tr>
      </tbody>
    </table>
  </div>
</template>

<style scoped>
.left {
  width: 48px;
  height: 24px;
  background: var(--white);
  border: 1px solid var(--ablack);
}

.table {
  border-collapse: collapse;
}

.mainContent {
  width: 24px;
  height: 24px;
  background: var(--white);
  border: 1px solid var(--ablack);
}
</style>

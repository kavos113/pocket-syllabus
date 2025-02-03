<script setup lang="ts">
import { ref } from 'vue';
import { Day, DAYS, Period, PERIODS } from '../../scripts/consts.ts';
import { SearchTimetableQuery } from './Search.vue';

const emits = defineEmits<{
  (event: 'checkItem', item: SearchTimetableQuery[]): void;
}>();

const days = ref<Day[]>(DAYS);
const periods = ref<Period[]>(PERIODS);

const checked = ref<Record<Day, Record<Period, boolean>>>({
  月: {
    '1': false,
    '2': false,
    '3': false,
    '4': false,
    '5': false,
  },
  火: {
    '1': false,
    '2': false,
    '3': false,
    '4': false,
    '5': false,
  },
  水: {
    '1': false,
    '2': false,
    '3': false,
    '4': false,
    '5': false,
  },
  木: {
    '1': false,
    '2': false,
    '3': false,
    '4': false,
    '5': false,
  },
  金: {
    '1': false,
    '2': false,
    '3': false,
    '4': false,
    '5': false,
  },
});

const onClick = (day: Day, period: Period) => {
  checked.value[day][period] = !checked.value[day][period];

  const checkedItems: SearchTimetableQuery[] = [];
  for (const day of days.value) {
    for (const period of periods.value) {
      if (checked.value[day][period]) {
        checkedItems.push({ day, period });
      }
    }
  }
  emits('checkItem', checkedItems);
};

const onClickDay = (day: Day) => {
  for (const period of periods.value) {
    checked.value[day][period] = !checked.value[day][period];
  }

  const checkedItems: SearchTimetableQuery[] = [];
  for (const day of days.value) {
    for (const period of periods.value) {
      if (checked.value[day][period]) {
        checkedItems.push({ day, period });
      }
    }
  }
  emits('checkItem', checkedItems);
};

const onClickPeriod = (period: Period) => {
  for (const day of days.value) {
    checked.value[day][period] = !checked.value[day][period];
  }

  const checkedItems: SearchTimetableQuery[] = [];
  for (const day of days.value) {
    for (const period of periods.value) {
      if (checked.value[day][period]) {
        checkedItems.push({ day, period });
      }
    }
  }
  emits('checkItem', checkedItems);
};
</script>

<template>
  <div class="timetable-container">
    <table class="table">
      <thead>
        <tr class="days">
          <th class="left"></th>
          <th
            v-for="day in days"
            :key="day"
            class="mainContent day"
            :class="`day-${day}`"
            @click="onClickDay(day)"
          >
            {{ day }}
          </th>
        </tr>
      </thead>
      <tbody>
        <tr
          v-for="period in periods"
          :key="period"
          class="row"
        >
          <th
            class="left period"
            @click="onClickPeriod(period)"
          >
            {{ period }}
          </th>
          <td
            v-for="day in days"
            :key="day"
            class="mainContent box"
            :class="[checked[day][period] ? 'checked' : '', `box-${day}`]"
            @click="onClick(day, period)"
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

.box:hover {
  background: var(--stmain);
  opacity: 0.3;
}

.checked {
  background: var(--stmain);
  cursor: pointer;
}

.day {
  cursor: pointer;
}

.period {
  cursor: pointer;
}

.box {
  cursor: pointer;
}

.row:has(.period:hover) > .box {
  background: var(--stmain);
  opacity: 0.3;
}

.table:has(.day-月:hover) .box-月 {
  background: var(--stmain);
  opacity: 0.3;
}

.table:has(.day-火:hover) .box-火 {
  background: var(--stmain);
  opacity: 0.3;
}

.table:has(.day-水:hover) .box-水 {
  background: var(--stmain);
  opacity: 0.3;
}

.table:has(.day-木:hover) .box-木 {
  background: var(--stmain);
  opacity: 0.3;
}

.table:has(.day-金:hover) .box-金 {
  background: var(--stmain);
  opacity: 0.3;
}
</style>

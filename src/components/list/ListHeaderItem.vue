<script setup lang="ts">
const emits = defineEmits<{
  (event: 'sort', key: string): void;
}>();

const onSort = (key: string) => {
  emits('sort', key);
};
</script>

<template>
  <div class="item-wrapper">
    <div class="item university">
      <p class="text">大学名</p>
    </div>
    <div
      class="item code"
      @click="onSort('code')"
    >
      <p class="text">コード</p>
    </div>
    <div
      class="item name"
      @click="onSort('title')"
    >
      <p class="text">講義名</p>
    </div>
    <div
      class="item lecturer"
      @click="onSort('lecturer')"
    >
      <p class="text">担当</p>
    </div>
    <div
      class="item timetable"
      @click="onSort('timetable')"
    >
      <p class="text">時間割</p>
    </div>
    <div class="item semester">
      <p class="text">開講時期</p>
    </div>
    <div
      class="item department"
      @click="onSort('department')"
    >
      <p class="text">開講元</p>
    </div>
    <div class="item credit">
      <p class="text">単位数</p>
    </div>
  </div>
</template>

<style scoped>
.item {
  border: 1px solid var(--ablack);
  height: 32px;
  display: flex;
  align-items: center;
}

.text {
  color: var(--3black, #000);
  font-family: 'Rounded Mplus 1c Medium', var(--font), sans-serif;
  font-size: 14px;
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
  cursor: pointer;
}

.name {
  grid-column: 5 / 12;
  grid-row: 1 / 2;
  cursor: pointer;
}

.lecturer {
  grid-column: 12 / 15;
  grid-row: 1 / 2;
  cursor: pointer;
}

.timetable {
  grid-column: 15 / 18;
  grid-row: 1 / 2;
  cursor: pointer;
}

.semester {
  grid-column: 18 / 20;
  grid-row: 1 / 2;
}

.department {
  grid-column: 20 / 23;
  grid-row: 1 / 2;
  cursor: pointer;
}

.credit {
  grid-column: 23 / 25;
  grid-row: 1 / 2;
}

@media (max-width: 600px) {
  .item-wrapper {
    display: grid;
    grid-template-columns: repeat(10, 1fr);
    grid-template-rows: 1fr;
  }

  .university {
    display: none;
  }

  .code {
    display: none;
  }

  .name {
    grid-column: 1 / 6;
    grid-row: 1 / 2;
    cursor: pointer;
  }

  .lecturer {
    display: none;
  }

  .timetable {
    grid-column: 6 / 8;
    grid-row: 1 / 2;
  }

  .semester {
    grid-column: 8 / 9;
    grid-row: 1 / 2;
    overflow: hidden;
    text-wrap: nowrap;
  }

  .department {
    grid-column: 9 / 11;
    grid-row: 1 / 2;
    cursor: pointer;
  }

  .credit {
    display: none;
  }
}
</style>

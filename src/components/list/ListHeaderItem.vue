<script setup lang="ts"></script>

<template>
  <div class="container">
    <div class="item university">
      <p class="text">大学名</p>
    </div>
    <div class="item code">
      <p class="text">コード</p>
    </div>
    <div class="item name">
      <p class="text">講義名</p>
    </div>
    <div class="item lecturer">
      <p class="text">担当</p>
    </div>
    <div class="item timetable">
      <p class="text">時間割</p>
    </div>
    <div class="item semester">
      <p class="text">開講時期</p>
    </div>
    <div class="item department">
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

.container {
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

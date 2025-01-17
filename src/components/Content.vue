<script setup lang="ts">
import ListTable from './list/ListTable.vue';
import Search from './search/Search.vue';
</script>

<template>
  <div class="container">
    <ListTable class="table" />
    <Search class="search" />
  </div>
</template>

<style scoped>
.container {
  display: grid;
  width: 100%;
  height: calc(100% - 74px);
  align-items: center;
  gap: 20px;
  grid-template-columns: repeat(3, 1fr);
  grid-template-rows: 1fr;
}

.table {
  grid-column: 1 / 3;
  grid-row: 1 / 2;
}

.search {
  grid-column: 3 / 4;
  grid-row: 1 / 2;
}
</style>

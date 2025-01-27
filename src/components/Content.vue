<script setup lang="ts">
import ListTable from './list/ListTable.vue';
import Search from './search/Search.vue';
</script>

<template>
  <div class="content-container">
    <Search class="search" />
    <ListTable class="table" />
  </div>
</template>

<style scoped>
.content-container {
  display: grid;
  width: 100%;
  height: calc(100% - 74px);
  align-items: center;
  gap: 20px;
  grid-template-columns: repeat(3, 1fr);
  grid-template-rows: 1fr;
}

.table {
  grid-column: 2 / 4;
  grid-row: 1 / 2;
}

.search {
  grid-column: 1 / 2;
  grid-row: 1 / 2;
}
</style>

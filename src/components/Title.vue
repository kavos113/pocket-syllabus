<script setup lang="ts"></script>

<template>
  <div class="container">
    <p class="title">Pocket Syllabus</p>
  </div>
</template>

<style scoped>
.container {
  width: 100%;
  height: 65px;
}

.title {
  width: 100%;
  height: 60px;
  flex-shrink: 0;
  color: var(--3black);
  font-family: 'M PLUS 1p Medium', var(--font), sans-serif;
  font-size: 40px;
  font-style: normal;
  font-weight: 700;
  line-height: normal;
  border-bottom: 4px solid var(--stmain);
  margin: 0;
}

.title::after {
  content: '';
  display: block;
  width: 100%;
  height: 2px;
  background-color: var(--stsub);
  margin-top: 6px;
}
</style>

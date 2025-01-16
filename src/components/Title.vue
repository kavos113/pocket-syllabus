<script setup lang="ts"></script>

<template>
  <div class="container">
    <p class="title">Pocket Syllabus</p>
  </div>
</template>

<style scoped>
.title {
  width: 100%;
  height: 60px;
  flex-shrink: 0;
  color: var(--3black);
  font-family: 'Mplus 1p Bold', var(--font), sans-serif;
  font-size: 40px;
  font-style: normal;
  font-weight: 700;
  line-height: normal;
  border-bottom: 2px solid var(--stsub);
  margin: 0;
}

.title::after {
  content: '';
  display: block;
  width: 100%;
  height: 4px;
  background-color: var(--stmain);
  bottom: 0;
  left: 0;
}
</style>

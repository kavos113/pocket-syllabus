<script setup lang="ts">
import ComboBox from '../common/ComboBox.vue';
import SearchBox from '../common/SearchBox.vue';
import {
  DEPARTMENTS,
  MOBILE_DEPARTMENTS,
  UNIVERSITIES,
  YEARS,
} from '../../scripts/consts.ts';
import { SearchComboBox } from './Search.vue';
import { ref, watch } from 'vue';

const emit = defineEmits<{
  (event: 'clickMenuItem', key: SearchComboBox, items: string[]): void;
  (event: 'changeSearchBox', title: string, lecturer: string): void;
}>();

const onUniversitySelect = (items: string[]) => {
  emit('clickMenuItem', 'university', items);
};

const onDepartmentSelect = (items: string[]) => {
  emit('clickMenuItem', 'department', items);
};

const onYearSelect = (items: string[]) => {
  emit('clickMenuItem', 'year', items);
};

const title = ref<string>('');
const lecturer = ref<string>('');

watch([title, lecturer], () => {
  emit('changeSearchBox', title.value, lecturer.value);
});
</script>

<template>
  <div class="search-box-wrapper">
    <ComboBox
      :items="UNIVERSITIES"
      @select-item="onUniversitySelect"
    />
    <ComboBox
      :items="DEPARTMENTS"
      class="desktop"
      @select-item="onDepartmentSelect"
    />
    <ComboBox
      :items="MOBILE_DEPARTMENTS"
      class="mobile"
      @select-item="onDepartmentSelect"
    />
    <ComboBox
      :items="YEARS"
      @select-item="onYearSelect"
    />
    <SearchBox
      v-model="title"
      placeholder="講義名"
    />
    <SearchBox
      v-model="lecturer"
      placeholder="教員名"
    />
  </div>
</template>

<style scoped>
.search-box-wrapper {
  display: flex;
  width: 100%;
  flex-direction: column;
  align-items: flex-start;
  gap: 24px;
}

.desktop {
  display: block;
}

.mobile {
  display: none;
}

@media (max-width: 600px) {
  .desktop {
    display: none;
  }

  .mobile {
    display: block;
  }
}
</style>

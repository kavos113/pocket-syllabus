<script setup lang="ts">
import { Course, CourseListItem } from '../../scripts/course.ts';
import ListHeaderItem from './ListHeaderItem.vue';
import ListItem from './ListItem.vue';
import CourseDetail from './CourseDetail.vue';
import { ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import SimpleButton from '../common/SimpleButton.vue';

const props = defineProps<{
  items: CourseListItem[];
}>();

const emits = defineEmits<{
  (event: 'sort', key: string): void;
}>();

const onSort = (key: string) => {
  emits('sort', key);
};

const detailsData = ref<Course>({
  id: 0,
  university: '',
  title: '',
  englishTitle: '',
  department: '',
  lecturer: [],
  lectureType: '',
  timeTable: [],
  code: '',
  credit: 0,
  year: 0,
  semester: [],
  language: '',
  url: '',
  sylbs_update: '',
  courseDetail: {
    abst: '',
    goal: '',
    experience: false,
    keyword: [],
    competencies: [],
    flow: '',
    schedule: [],
    outOfClass: '',
    textbook: '',
    referenceBook: '',
    assessment: '',
    relatedCourse: [],
    prerequisite: '',
    contact: '',
    officeHour: '',
    note: '',
  },
});

const isDetailOpen = ref<boolean>(false);
const isOverlayActive = ref<boolean>(false);

const onListItemClick = async (id: number) => {
  await invoke('get_course', { id }).then((data) => {
    detailsData.value = data as Course;
  });
  isDetailOpen.value = true;
  isOverlayActive.value = true;
};

const closeDetail = async () => {
  isDetailOpen.value = false;
  await new Promise((resolve) => setTimeout(resolve, 250));
  isOverlayActive.value = false;
};
</script>

<template>
  <div class="table">
    <ListHeaderItem @sort="onSort" />
    <div
      v-for="item in props.items"
      :key="item.id"
    >
      <ListItem
        :item="item"
        @click="onListItemClick(item.id)"
        class="item"
      />
    </div>
    <CourseDetail
      class="detail"
      :class="{ detailActive: isDetailOpen }"
      :content="detailsData as Course"
    />
    <SimpleButton
      text="戻る"
      class="back-motomoto"
      :class="{ back: isDetailOpen }"
      @click="closeDetail"
    />
    <div
      class="overlay"
      :class="{
        overlayActive: isDetailOpen,
        overlayActiveZIndex: isOverlayActive,
      }"
      @click="closeDetail"
    ></div>
  </div>
</template>

<style scoped>
.table {
  overflow-y: auto;
  height: 100%;
}

.detail {
  position: fixed;
  top: 0;
  left: 0;
  background-color: var(--white);
  width: 60%;
  height: 100%;
  overflow-y: auto;
  z-index: 100;
  transform: translateX(-100%);
  transition-property: transform;
  transition-duration: 0.25s;
}

.detailActive {
  transform: translateX(0);
}

.overlay {
  position: fixed;
  top: 0;
  left: 0;
  background-color: rgba(0, 0, 0, 0.3);
  width: 100%;
  height: 100%;
  z-index: -1;
  opacity: 0;
  transition-property: opacity;
  transition-duration: 0.25s;
}

.overlayActive {
  z-index: 1;
  opacity: 1;
}

.overlayActiveZIndex {
  z-index: 1;
}

.item:hover {
  box-shadow: 0 0 15px rgba(0, 0, 0, 0.5);
}

.item {
  cursor: pointer;
}

.back {
  display: none;
}

.back-motomoto {
  display: none;
}

@media (max-width: 600px) {
  .back {
    display: block;
    position: fixed;
    bottom: 10px;
    right: 10px;
    z-index: 101;
    box-shadow: 0 0 15px var(--stmain);
  }

  .detail {
    width: 100%;
    transform: translateX(100%);
  }

  .detailActive {
    transform: translateX(0);
  }
}
</style>

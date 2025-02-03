<script setup lang="ts">
import CheckBox from '../common/CheckBox.vue';
import { onMounted, ref, watch } from 'vue';

const props = defineProps<{
  checkboxId: string;
  contents: string[];
}>();

const emits = defineEmits<{
  (event: 'checkItem', items: string[]): void;
}>();

onMounted(() => {
  const checkedArray: boolean[] = [];
  props.contents.forEach(() => {
    checkedArray.push(false);
  });
  checked.value = checkedArray;
});

const checked = ref<boolean[]>([]);

watch(
  checked,
  (newValue, oldValue) => {
    const checkedItems: string[] = [];
    newValue.forEach((value, index) => {
      if (value) {
        checkedItems.push(props.contents[index]);
      }
    });
    emits('checkItem', checkedItems);
  },
  { deep: true },
);

const onCheck = (index: number, value: boolean) => {
  checked.value[index - 1] = value;
};
</script>

<template>
  <div class="check-boxes-container">
    <CheckBox
      v-for="index in props.contents.length"
      :key="index"
      :checkbox-id="props.checkboxId + index"
      :content="props.contents[index - 1]"
      @check-item="onCheck"
    />
  </div>
</template>

<style scoped></style>

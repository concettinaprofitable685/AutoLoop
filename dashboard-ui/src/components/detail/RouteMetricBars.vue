<script setup lang="ts">
defineProps<{
  title: string;
  items: Array<{ label: string; value: number; max?: number }>;
}>();

function widthPercent(value: number, max?: number) {
  const denominator = max && max > 0 ? max : 1;
  return `${Math.max(8, Math.round((value / denominator) * 100))}%`;
}
</script>

<template>
  <div class="detail-stack">
    <strong>{{ title }}</strong>
    <div class="bar-list">
      <div v-for="item in items" :key="item.label" class="bar-row">
        <div class="bar-label-row">
          <span>{{ item.label }}</span>
          <strong>{{ item.value }}</strong>
        </div>
        <div class="bar-track">
          <div class="bar-fill" :style="{ width: widthPercent(item.value, item.max) }"></div>
        </div>
      </div>
    </div>
  </div>
</template>

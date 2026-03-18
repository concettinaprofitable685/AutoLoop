<script setup lang="ts">
defineProps<{
  label: string;
  value: string;
  tone?: "default" | "good" | "warn";
  caption?: string;
}>();
</script>

<template>
  <article class="metric-tile" :data-tone="tone ?? 'default'">
    <div class="metric-label">{{ label }}</div>
    <div class="metric-value">{{ value }}</div>
    <div v-if="caption" class="metric-caption">{{ caption }}</div>
  </article>
</template>

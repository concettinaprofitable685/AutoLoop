<script setup lang="ts">
import MetricTile from "../MetricTile.vue";

defineProps<{
  label: string;
  value: string;
  caption?: string;
  tone?: "good" | "warn" | "bad";
}>();
</script>

<template>
  <MetricTile :label="label" :value="value" :caption="caption" :tone="tone" />
</template>

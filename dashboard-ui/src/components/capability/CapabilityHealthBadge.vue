<script setup lang="ts">
defineProps<{
  value: number;
}>();
</script>

<template>
  <span class="health-badge">{{ value.toFixed(2) }}</span>
</template>

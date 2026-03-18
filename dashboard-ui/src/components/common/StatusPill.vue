<script setup lang="ts">
defineProps<{
  value: string;
}>();
</script>

<template>
  <span class="status-pill">{{ value }}</span>
</template>

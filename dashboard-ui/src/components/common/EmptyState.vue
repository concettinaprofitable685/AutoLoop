<script setup lang="ts">
defineProps<{
  message: string;
}>();
</script>

<template>
  <div class="empty-state">{{ message }}</div>
</template>

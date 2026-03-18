<script setup lang="ts">
defineProps<{
  sessionId: string;
  anchor: string;
}>();
</script>

<template>
  <div class="session-badge">
    <span>{{ sessionId }}</span>
    <span>{{ anchor }}</span>
  </div>
</template>

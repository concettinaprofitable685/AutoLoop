<script setup lang="ts">
defineProps<{
  source: string;
  identifier: string;
}>();
</script>

<template>
  <div class="data-source-card">
    <strong>{{ source }}</strong>
    <p>{{ identifier }}</p>
  </div>
</template>

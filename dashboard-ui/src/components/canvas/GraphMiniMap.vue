<script setup lang="ts">
defineProps<{
  nodeCount: number;
  edgeCount: number;
}>();
</script>

<template>
  <div class="graph-minimap">
    <span>{{ nodeCount }} nodes</span>
    <span>{{ edgeCount }} edges</span>
  </div>
</template>

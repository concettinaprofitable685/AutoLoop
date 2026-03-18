<script setup lang="ts">
import SearchInput from "../common/SearchInput.vue";

defineProps<{
  searchQuery: string;
}>();

const emit = defineEmits<{
  search: [value: string];
  "clear-search": [];
}>();
</script>

<template>
  <div class="graph-toolbar">
    <SearchInput :model-value="searchQuery" @update:model-value="emit('search', $event)" />
    <button class="action-button secondary" @click="emit('clear-search')">Clear</button>
  </div>
</template>

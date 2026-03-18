<script setup lang="ts">
const model = defineModel<string>({ default: "" });
</script>

<template>
  <input v-model="model" class="search-input" type="search" placeholder="Search nodes or capabilities" />
</template>

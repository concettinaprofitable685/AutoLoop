<script setup lang="ts">
defineProps<{
  label: string;
  value: string;
}>();
</script>

<template>
  <label class="field-label">
    {{ label }}
    <input :value="value" />
  </label>
</template>

<script setup lang="ts">
defineProps<{
  index: number;
  title: string;
  payload: Record<string, unknown>;
  selected?: boolean;
}>();

const emit = defineEmits<{
  selectEvent: [payload: { index: number; tool?: string }];
}>();
</script>

<template>
  <article
    class="event-card"
    :data-selected="selected ? 'true' : 'false'"
    @click="emit('selectEvent', { index, tool: payload.tool ? String(payload.tool) : undefined })"
  >
    <strong>{{ title }}</strong>
    <pre>{{ JSON.stringify(payload, null, 2) }}</pre>
  </article>
</template>

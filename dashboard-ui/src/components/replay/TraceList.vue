<script setup lang="ts">
import StatusPill from "../common/StatusPill.vue";
import type { ReplayTrace } from "../../types/replay";

defineProps<{
  traces: ReplayTrace[];
}>();
</script>

<template>
  <ul class="stack-list compact">
    <li v-for="trace in traces" :key="`${trace.span_name}-${trace.detail}`" class="stack-item">
      <div>
        <strong>{{ trace.span_name }}</strong>
        <p>{{ trace.detail }}</p>
      </div>
      <StatusPill :value="trace.level" />
    </li>
  </ul>
</template>

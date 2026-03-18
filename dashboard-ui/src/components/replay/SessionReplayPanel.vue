<script setup lang="ts">
import ReplayTimeline from "./ReplayTimeline.vue";
import TraceList from "./TraceList.vue";
import type { SessionReplay } from "../../types/replay";

defineProps<{
  replay: SessionReplay | null;
  selectedIndex?: number;
}>();

const emit = defineEmits<{
  selectEvent: [payload: { index: number; tool?: string }];
}>();
</script>

<template>
  <div v-if="replay" class="detail-stack">
    <div class="data-source-card">
      <strong>Deliberation</strong>
      <p>Rounds: {{ replay.deliberation?.round_count ?? 0 }}</p>
      <p>Order: {{ replay.deliberation?.final_execution_order?.join(' -> ') || 'n/a' }}</p>
    </div>
    <ReplayTimeline
      :events="replay.executionFeedback"
      :selected-index="selectedIndex"
      @select-event="emit('selectEvent', $event)"
    />
    <TraceList :traces="replay.traces" />
  </div>
</template>

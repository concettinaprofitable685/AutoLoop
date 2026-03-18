<script setup lang="ts">
import SessionReplayPanel from "../replay/SessionReplayPanel.vue";
import RouteForensicsPanel from "./RouteForensicsPanel.vue";
import type { SessionReplay } from "../../types/replay";

defineProps<{
  replay: SessionReplay | null;
  routeForensics: {
    openCircuits: string[];
    failingTools: string[];
    treatmentShare: number;
  };
  selectedReplayTool?: string;
  selectedReplayIndex?: number;
}>();

const emit = defineEmits<{
  selectEvent: [payload: { index: number; tool?: string }];
}>();
</script>

<template>
  <div class="detail-stack">
    <RouteForensicsPanel
      :route-forensics="routeForensics"
      :replay="replay"
      :selected-replay-tool="selectedReplayTool"
    />
    <SessionReplayPanel
      :replay="replay"
      :selected-index="selectedReplayIndex"
      @select-event="emit('selectEvent', $event)"
    />
  </div>
</template>

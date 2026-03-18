<script setup lang="ts">
import { computed } from "vue";
import RouteMetricBars from "./RouteMetricBars.vue";

const props = defineProps<{
  routeForensics: {
    openCircuits: string[];
    failingTools: string[];
    treatmentShare: number;
  };
  replay: {
    routeAnalytics?: Record<string, unknown>;
    failureForensics?: Record<string, unknown>;
  } | null;
  selectedReplayTool?: string;
}>();

const openCircuits = computed(() => props.routeForensics?.openCircuits ?? []);
const failingTools = computed(() => props.routeForensics?.failingTools ?? []);
const treatmentShare = computed(() => props.routeForensics?.treatmentShare ?? 0);

const summaryBars = computed(() => [
  {
    label: "Treatment share",
    value: Math.round(treatmentShare.value * 100),
    max: 100
  },
  {
    label: "Open circuits",
    value: openCircuits.value.length,
    max: Math.max(1, openCircuits.value.length, failingTools.value.length)
  },
  {
    label: "Failing tools",
    value: failingTools.value.length,
    max: Math.max(1, openCircuits.value.length, failingTools.value.length)
  }
]);

const topToolBars = computed(() => {
  const tools = (props.replay?.routeAnalytics?.top_tools as string[] | undefined) ?? [];
  const guardedReports = Number(props.replay?.routeAnalytics?.guarded_reports ?? 0);
  return tools.slice(0, 5).map((label, index) => ({
    label,
    value: Math.max(1, guardedReports - index + 1),
    max: Math.max(guardedReports, tools.length, 1)
  }));
});

const failureBars = computed(() => {
  const blocked = (props.replay?.failureForensics?.blocked_tools as string[] | undefined) ?? [];
  const approval =
    (props.replay?.failureForensics?.approval_gated_tools as string[] | undefined) ?? [];
  return [...blocked, ...approval].slice(0, 5).map((label, index) => ({
    label,
    value: Math.max(1, 5 - index),
    max: 5
  }));
});
</script>

<template>
  <div class="detail-stack">
    <RouteMetricBars title="Route health" :items="summaryBars" />
    <div class="forensics-grid">
      <div class="data-source-card">
        <strong>Replay focus</strong>
        <p>{{ selectedReplayTool || "none" }}</p>
      </div>
      <div class="data-source-card">
        <strong>Open circuit names</strong>
        <p>{{ openCircuits.join(", ") || "none" }}</p>
      </div>
      <div class="data-source-card">
        <strong>Failing tool names</strong>
        <p>{{ failingTools.join(", ") || "none" }}</p>
      </div>
    </div>
    <RouteMetricBars
      v-if="topToolBars.length"
      title="Top route surfaces"
      :items="topToolBars"
    />
    <RouteMetricBars
      v-if="failureBars.length"
      title="Failure surfaces"
      :items="failureBars"
    />
  </div>
</template>

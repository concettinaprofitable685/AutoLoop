<script setup lang="ts">
import { computed, onMounted, ref, watch } from "vue";
import SummaryStrip from "../components/summary/SummaryStrip.vue";
import RuntimeSidebar from "../components/sidebar/RuntimeSidebar.vue";
import GraphCanvasShell from "../components/canvas/GraphCanvasShell.vue";
import DetailWorkbench from "../components/detail/DetailWorkbench.vue";
import { demoDashboardSnapshot, demoReplay } from "../data";
import { mapSnapshotToGraph, deriveAgents } from "../services/graphMapper";
import { buildDashboardUrl, DEFAULT_SNAPSHOT_BASE } from "../services/snapshotApi";
import { buildReplayUrl } from "../services/replayApi";
import { useDashboardSnapshot } from "../composables/useDashboardSnapshot";
import { useSessionReplay } from "../composables/useSessionReplay";
import { useGraphSelection } from "../composables/useGraphSelection";
import { useGraphFilters } from "../composables/useGraphFilters";
import { useCapabilityGovernance } from "../composables/useCapabilityGovernance";
import { useDashboardEvents } from "../composables/useDashboardEvents";

const snapshotState = useDashboardSnapshot(demoDashboardSnapshot);
const replayState = useSessionReplay(demoReplay);
const apiBaseUrl = ref(DEFAULT_SNAPSHOT_BASE);
const replayUrl = ref(buildReplayUrl(demoDashboardSnapshot.sessionId, apiBaseUrl.value));

const mergedSnapshot = computed(() => ({
  ...snapshotState.snapshot.value,
  replay: replayState.replay.value
}));
const graphModel = computed(() =>
  mapSnapshotToGraph(mergedSnapshot.value, replayState.replay.value)
);
const agents = computed(() => deriveAgents(mergedSnapshot.value, replayState.replay.value));
const selection = useGraphSelection(graphModel);
const filters = useGraphFilters(graphModel);
const governance = useCapabilityGovernance(mergedSnapshot);
const dashboardEvents = useDashboardEvents();
const selectedReplayTool = ref("");
const selectedReplayIndex = ref<number | undefined>(undefined);
const effectiveSnapshot = computed(() => ({
  ...mergedSnapshot.value,
  capabilityCatalog: governance.effectiveCapabilities.value
}));
const highlightedNodeIds = computed(() => {
  const ids = new Set<string>();
  if (selection.selectedNodeId.value) {
    ids.add(selection.selectedNodeId.value);
  }
  for (const node of selection.relatedNodes.value) {
    ids.add(node.id);
  }
  return Array.from(ids);
});
const lifecycleHotLabels = computed(() =>
  governance.lifecycleEntries.value.map((entry) => entry.tool_name)
);
const safeCanvasGraph = computed(() => filters.filteredGraph.value);
const safeHighlightedNodeIds = computed(() => highlightedNodeIds.value);
const safeRouteFocusLabels = computed(() => routeFocusLabels.value);
const summaryGovernance = computed(() => ({
  lifecycleEntries: governance.lifecycleEntries.value,
  routeForensics: governance.routeForensics.value
}));
const sidebarFilters = computed(() => ({
  entityFilters: filters.entityFilters.value,
  relationFilters: filters.relationFilters.value,
  selectedEntityFilters: filters.selectedEntityTypes.value,
  selectedRelationFilters: filters.selectedEdgeKinds.value
}));
const routeFocusLabels = computed(() => [
  ...(selectedReplayTool.value ? [selectedReplayTool.value] : []),
  ...governance.routeForensics.value.failingTools,
  ...governance.routeForensics.value.openCircuits
]);

async function refreshAll() {
  const sessionId = snapshotState.snapshot.value.sessionId;
  snapshotState.remoteUrl.value = buildDashboardUrl(sessionId, apiBaseUrl.value);
  replayUrl.value = buildReplayUrl(sessionId, apiBaseUrl.value);
  await snapshotState.loadFromUrl(snapshotState.remoteUrl.value);
  await replayState.loadFromUrl(replayUrl.value);
}

function clearGraphFocus() {
  selection.selectNode(null);
  selectedReplayTool.value = "";
  selectedReplayIndex.value = undefined;
  filters.reset();
}

async function governCapability(
  action: "verify" | "deprecate" | "rollback",
  tool: string
) {
  await governance.govern(apiBaseUrl.value, action, tool);
  await refreshAll();
}

function handleReplayToolSelection(tool: string) {
  selectedReplayTool.value = tool;
  const match = graphModel.value.nodes.find((node) => node.label === tool);
  if (match) {
    selection.selectNode(match.id);
  }
}

function handleReplayEventSelection(payload: { index: number; tool?: string }) {
  selectedReplayIndex.value = payload.index;
  if (payload.tool) {
    handleReplayToolSelection(payload.tool);
  }
}

onMounted(() => {
  snapshotState.remoteUrl.value = buildDashboardUrl(
    snapshotState.snapshot.value.sessionId,
    apiBaseUrl.value
  );
  replayUrl.value = buildReplayUrl(snapshotState.snapshot.value.sessionId, apiBaseUrl.value);
  dashboardEvents.connect(apiBaseUrl.value, async (payload) => {
    if (payload.kind === "capability_governed") {
      await refreshAll();
    }
  });
});

watch(apiBaseUrl, (value) => {
  dashboardEvents.connect(value, async (payload) => {
    if (payload.kind === "capability_governed") {
      await refreshAll();
    }
  });
});
</script>

<template>
  <div class="control-plane-page">
    <SummaryStrip
      :snapshot="effectiveSnapshot"
      :agents="agents"
      :graph="graphModel"
      :governance="summaryGovernance"
    />
    <div class="control-plane-grid">
      <RuntimeSidebar
        :agents="agents"
        :capabilities="effectiveSnapshot.capabilityCatalog"
        :sessions="[mergedSnapshot.sessionId]"
        :entity-filters="sidebarFilters.entityFilters"
        :relation-filters="sidebarFilters.relationFilters"
        :selected-entity-filters="sidebarFilters.selectedEntityFilters"
        :selected-relation-filters="sidebarFilters.selectedRelationFilters"
        :dashboard-url="apiBaseUrl"
        :replay-url="replayUrl"
        @toggle-entity-filter="filters.toggleEntityType"
        @toggle-relation-filter="filters.toggleEdgeKind"
        @dashboard-url-update="apiBaseUrl = $event"
        @replay-url-update="replayUrl = $event"
        @load-dashboard-url="refreshAll"
        @load-replay-url="replayState.loadFromUrl"
        @refresh="refreshAll"
        @clear-graph="clearGraphFocus"
      />
      <GraphCanvasShell
        :graph="safeCanvasGraph"
        :selected-node-id="selection.selectedNodeId.value"
        :search-query="filters.searchQuery.value"
        :highlighted-node-ids="safeHighlightedNodeIds"
        :lifecycle-hot-labels="lifecycleHotLabels"
        :route-focus-labels="safeRouteFocusLabels"
        :selected-replay-index="selectedReplayIndex"
        @select-node="selection.selectNode"
        @search="filters.setSearch"
        @clear-search="filters.setSearch('')"
      />
      <DetailWorkbench
        :snapshot="effectiveSnapshot"
        :replay="replayState.replay.value"
        :selected-node="selection.selectedNode.value"
        :related-nodes="selection.relatedNodes.value"
        :capabilities="effectiveSnapshot.capabilityCatalog"
        :capability-lifecycle="governance.lifecycleEntries.value"
        :route-forensics="governance.routeForensics.value"
        :pending-tool="governance.pendingTool.value"
        :selected-replay-tool="selectedReplayTool"
        :selected-replay-index="selectedReplayIndex"
        @govern="governCapability"
        @select-replay-event="handleReplayEventSelection"
      />
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from "vue";
import PanelFrame from "../common/PanelFrame.vue";
import EmptyState from "../common/EmptyState.vue";
import GraphToolbar from "./GraphToolbar.vue";
import GraphLegend from "./GraphLegend.vue";
import RouteOverlayLegend from "./RouteOverlayLegend.vue";
import GraphMiniMap from "./GraphMiniMap.vue";
import GraphCanvas from "./GraphCanvas.vue";
import type { UiGraphModel } from "../../types/graph";
import { useUiPreferences } from "../../composables/useUiPreferences";

const props = defineProps<{
  graph: UiGraphModel;
  selectedNodeId: string | null;
  searchQuery: string;
  highlightedNodeIds: string[];
  lifecycleHotLabels: string[];
  routeFocusLabels: string[];
  selectedReplayIndex?: number;
}>();

const emit = defineEmits<{
  selectNode: [nodeId: string];
  search: [value: string];
  "clear-search": [];
}>();

const safeGraph = computed<UiGraphModel>(() => ({
  nodes: Array.isArray(props.graph?.nodes) ? props.graph.nodes : [],
  edges: Array.isArray(props.graph?.edges) ? props.graph.edges : []
}));
const nodeCount = computed(() => safeGraph.value.nodes.length);
const edgeCount = computed(() => safeGraph.value.edges.length);
const { t } = useUiPreferences();
</script>

<template>
  <PanelFrame :title="t('canvasTitle')" :subtitle="t('canvasSubtitle')">
    <template #actions>
      <GraphMiniMap :node-count="nodeCount" :edge-count="edgeCount" />
    </template>
    <GraphToolbar :search-query="searchQuery" @search="emit('search', $event)" @clear-search="emit('clear-search')" />
    <GraphLegend />
    <RouteOverlayLegend />
    <EmptyState v-if="nodeCount === 0" message="No graph nodes match the current filters." />
    <GraphCanvas
      v-else
      :graph="safeGraph"
      :selected-node-id="selectedNodeId"
      :highlighted-node-ids="highlightedNodeIds"
      :lifecycle-hot-labels="lifecycleHotLabels"
      :route-focus-labels="routeFocusLabels"
      :selected-replay-index="selectedReplayIndex"
      @select-node="emit('selectNode', $event)"
    />
  </PanelFrame>
</template>

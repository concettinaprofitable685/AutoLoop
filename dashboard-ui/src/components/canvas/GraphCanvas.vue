<script setup lang="ts">
import type { UiGraphModel } from "../../types/graph";

defineProps<{
  graph: UiGraphModel;
  selectedNodeId: string | null;
  highlightedNodeIds: string[];
  lifecycleHotLabels: string[];
  routeFocusLabels: string[];
  selectedReplayIndex?: number;
}>();

const emit = defineEmits<{
  selectNode: [nodeId: string];
}>();

function nodeColor(kind: string): string {
  switch (kind) {
    case "anchor":
      return "#f26b1d";
    case "agent":
      return "#0d9488";
    case "capability":
      return "#2563eb";
    case "verifier":
      return "#dc2626";
    case "runtime":
      return "#7c3aed";
    default:
      return "#475569";
  }
}

function isHighlighted(nodeId: string, highlightedNodeIds: string[]): boolean {
  return highlightedNodeIds.length === 0 || highlightedNodeIds.includes(nodeId);
}

function isLifecycleHot(label: string, lifecycleHotLabels: string[]): boolean {
  return lifecycleHotLabels.includes(label);
}

function isRouteFocused(
  label: string,
  routeFocusLabels: string[],
  kind: string,
): boolean {
  return kind === "runtime" || routeFocusLabels.some((entry) => label.includes(entry));
}
</script>

<template>
  <svg class="graph-canvas" viewBox="0 0 1080 640" role="img" aria-label="AutoLoop graph canvas">
    <line
      v-for="edge in graph.edges"
      :key="edge.id"
      :x1="graph.nodes.find((node) => node.id === edge.source)?.x ?? 0"
      :y1="graph.nodes.find((node) => node.id === edge.source)?.y ?? 0"
      :x2="graph.nodes.find((node) => node.id === edge.target)?.x ?? 0"
      :y2="graph.nodes.find((node) => node.id === edge.target)?.y ?? 0"
      class="graph-edge"
      :class="{ 'graph-edge-active': selectedReplayIndex !== undefined && selectedReplayIndex !== null && edge.kind === 'routes' }"
      :data-kind="edge.kind"
      :stroke="
        selectedReplayIndex !== undefined && selectedReplayIndex !== null && edge.kind === 'routes'
          ? '#0f766e'
          : undefined
      "
      :stroke-width="
        selectedReplayIndex !== undefined && selectedReplayIndex !== null && edge.kind === 'routes'
          ? 4
          : 2
      "
      :opacity="
        highlightedNodeIds.length === 0 ||
        highlightedNodeIds.includes(edge.source) ||
        highlightedNodeIds.includes(edge.target)
          ? 1
          : 0.2
      "
    />
    <g
      v-for="node in graph.nodes"
      :key="node.id"
      class="graph-node"
      :class="{ 'graph-node-active': isRouteFocused(node.label, routeFocusLabels, node.kind) }"
      :transform="`translate(${node.x}, ${node.y})`"
      :opacity="isHighlighted(node.id, highlightedNodeIds) ? 1 : 0.22"
      @click="emit('selectNode', node.id)"
    >
      <circle
        :r="selectedNodeId === node.id ? 28 : 22"
        :fill="nodeColor(node.kind)"
        :stroke="
          selectedNodeId === node.id
            ? '#f8fafc'
            : isLifecycleHot(node.label, lifecycleHotLabels)
              ? '#d97706'
              : isRouteFocused(node.label, routeFocusLabels, node.kind)
                ? '#0f766e'
                : 'transparent'
        "
        stroke-width="3"
      />
      <text y="46" text-anchor="middle">{{ node.label }}</text>
    </g>
  </svg>
</template>

<script setup lang="ts">
import EmptyState from "../common/EmptyState.vue";
import DataSourceCard from "./DataSourceCard.vue";
import RelatedNodeList from "./RelatedNodeList.vue";
import type { UiGraphNode } from "../../types/graph";

defineProps<{
  selectedNode: UiGraphNode | null;
  relatedNodes: UiGraphNode[];
}>();
</script>

<template>
  <EmptyState v-if="!selectedNode" message="Select a node on the graph to inspect metadata." />
  <div v-else class="detail-stack">
    <div>
      <strong>{{ selectedNode.label }}</strong>
      <p>ID: {{ selectedNode.id }}</p>
      <p>Type: {{ selectedNode.kind }}</p>
      <p>Status: {{ selectedNode.status ?? "n/a" }}</p>
    </div>
    <DataSourceCard :source="selectedNode.source ?? 'unknown'" :identifier="selectedNode.id" />
    <RelatedNodeList :related-nodes="relatedNodes" />
  </div>
</template>

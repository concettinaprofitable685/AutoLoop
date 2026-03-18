<script setup lang="ts">
import StatusPill from "../common/StatusPill.vue";
import type { UiGraphNode } from "../../types/graph";

defineProps<{
  relatedNodes: UiGraphNode[];
}>();
</script>

<template>
  <div>
    <h3>Related nodes</h3>
    <ul class="stack-list compact">
      <li v-for="node in relatedNodes" :key="node.id" class="stack-item">
        <strong>{{ node.label }}</strong>
        <StatusPill :value="node.kind" />
      </li>
    </ul>
  </div>
</template>

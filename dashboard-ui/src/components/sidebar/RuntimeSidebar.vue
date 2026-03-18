<script setup lang="ts">
import AgentListPanel from "./AgentListPanel.vue";
import CapabilityListPanel from "./CapabilityListPanel.vue";
import SessionListPanel from "./SessionListPanel.vue";
import GraphFilterPanel from "./GraphFilterPanel.vue";
import GlobalActionsPanel from "./GlobalActionsPanel.vue";
import type { AgentRecord } from "../../types/agent";
import type { CapabilityRecord } from "../../types/capability";

defineProps<{
  agents: AgentRecord[];
  capabilities: CapabilityRecord[];
  sessions: string[];
  entityFilters: string[];
  relationFilters: string[];
  selectedEntityFilters: string[];
  selectedRelationFilters: string[];
  dashboardUrl: string;
  replayUrl: string;
}>();

const emit = defineEmits<{
  toggleEntityFilter: [kind: string];
  toggleRelationFilter: [kind: string];
  "dashboard-url-update": [value: string];
  "replay-url-update": [value: string];
  "load-dashboard-url": [value: string];
  "load-replay-url": [value: string];
  refresh: [];
  "clear-graph": [];
}>();
</script>

<template>
  <aside class="runtime-sidebar">
    <AgentListPanel :agents="agents" />
    <CapabilityListPanel :capabilities="capabilities" />
    <SessionListPanel :sessions="sessions" />
    <GraphFilterPanel
      :entity-filters="entityFilters"
      :relation-filters="relationFilters"
      :selected-entity-filters="selectedEntityFilters"
      :selected-relation-filters="selectedRelationFilters"
      @toggle-entity-filter="emit('toggleEntityFilter', $event)"
      @toggle-relation-filter="emit('toggleRelationFilter', $event)"
    />
    <GlobalActionsPanel
      :dashboard-url="dashboardUrl"
      :replay-url="replayUrl"
      @dashboard-url-update="emit('dashboard-url-update', $event)"
      @replay-url-update="emit('replay-url-update', $event)"
      @load-dashboard-url="emit('load-dashboard-url', $event)"
      @load-replay-url="emit('load-replay-url', $event)"
      @refresh="emit('refresh')"
      @clear-graph="emit('clear-graph')"
    />
  </aside>
</template>

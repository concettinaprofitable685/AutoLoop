<script setup lang="ts">
import PanelFrame from "../common/PanelFrame.vue";
import StatusPill from "../common/StatusPill.vue";
import type { AgentRecord } from "../../types/agent";
import { useUiPreferences } from "../../composables/useUiPreferences";

defineProps<{
  agents: AgentRecord[];
}>();

const { t } = useUiPreferences();
</script>

<template>
  <PanelFrame :title="t('agentsTitle')" :subtitle="t('agentsSubtitle')">
    <ul class="stack-list">
      <li v-for="agent in agents" :key="agent.id" class="stack-item">
        <div>
          <strong>{{ agent.name }}</strong>
          <p>{{ agent.phase }}</p>
        </div>
        <div class="stack-meta">
          <StatusPill :value="agent.status" />
          <span>{{ agent.reputation.toFixed(2) }}</span>
        </div>
      </li>
    </ul>
  </PanelFrame>
</template>

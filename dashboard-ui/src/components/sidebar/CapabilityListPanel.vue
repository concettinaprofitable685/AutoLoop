<script setup lang="ts">
import PanelFrame from "../common/PanelFrame.vue";
import StatusPill from "../common/StatusPill.vue";
import type { CapabilityRecord } from "../../types/capability";
import { useUiPreferences } from "../../composables/useUiPreferences";

defineProps<{
  capabilities: CapabilityRecord[];
}>();

const { t } = useUiPreferences();
</script>

<template>
  <PanelFrame :title="t('capabilitiesTitle')" :subtitle="t('capabilitiesSubtitle')">
    <ul class="stack-list compact">
      <li v-for="capability in capabilities.slice(0, 6)" :key="capability.name" class="stack-item">
        <div>
          <strong>{{ capability.name }}</strong>
          <p>{{ capability.scope }} | {{ capability.risk }}</p>
        </div>
        <div class="stack-meta">
          <StatusPill :value="capability.status" />
          <span>{{ capability.health.toFixed(2) }}</span>
        </div>
      </li>
    </ul>
  </PanelFrame>
</template>

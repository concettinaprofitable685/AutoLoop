<script setup lang="ts">
import ParamEditor from "./ParamEditor.vue";
import CapabilityTable from "../capability/CapabilityTable.vue";
import CapabilityLifecycleDrawer from "../capability/CapabilityLifecycleDrawer.vue";
import type {
  CapabilityGovernanceAction,
  CapabilityRecord,
  CapabilityLifecycleEntry
} from "../../types/capability";

defineProps<{
  capabilities: CapabilityRecord[];
  capabilityLifecycle: CapabilityLifecycleEntry[];
  pendingTool?: string;
}>();

const emit = defineEmits<{
  govern: [action: CapabilityGovernanceAction, tool: string];
}>();
</script>

<template>
  <div class="detail-stack">
    <div class="form-stack">
      <ParamEditor label="Verifier gate" value="strict" />
      <ParamEditor label="Breaker cooldown (ms)" value="300000" />
      <ParamEditor label="Evolution cycle" value="daily" />
    </div>
    <CapabilityTable
      :capabilities="capabilities"
      :pending-tool="pendingTool"
      @govern="(action, tool) => emit('govern', action, tool)"
    />
    <CapabilityLifecycleDrawer :entries="capabilityLifecycle" />
  </div>
</template>

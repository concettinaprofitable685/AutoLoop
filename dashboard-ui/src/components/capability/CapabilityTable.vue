<script setup lang="ts">
import { computed } from "vue";
import CapabilityHealthBadge from "./CapabilityHealthBadge.vue";
import StatusPill from "../common/StatusPill.vue";
import type { CapabilityGovernanceAction, CapabilityRecord } from "../../types/capability";

const props = defineProps<{
  capabilities: CapabilityRecord[];
  pendingTool?: string;
}>();

const emit = defineEmits<{
  govern: [action: CapabilityGovernanceAction, tool: string];
}>();

const safeCapabilities = computed(() => props.capabilities ?? []);
</script>

<template>
  <div class="table-shell">
    <table class="capability-table">
      <thead>
        <tr>
          <th>Name</th>
          <th>Status</th>
          <th>Approval</th>
          <th>Scope</th>
          <th>Health</th>
          <th>Actions</th>
        </tr>
      </thead>
      <tbody>
        <tr v-for="capability in safeCapabilities" :key="capability.name">
          <td>{{ capability.name }}</td>
          <td><StatusPill :value="capability.status" /></td>
          <td>{{ capability.approval }}</td>
          <td>{{ capability.scope }}</td>
          <td><CapabilityHealthBadge :value="capability.health" /></td>
          <td>
            <div class="chip-row">
              <button
                class="action-button secondary"
                :disabled="pendingTool === capability.name"
                @click="emit('govern', 'verify', capability.name)"
              >
                Verify
              </button>
              <button
                class="action-button secondary"
                :disabled="pendingTool === capability.name"
                @click="emit('govern', 'rollback', capability.name)"
              >
                Rollback
              </button>
              <button
                class="action-button secondary"
                :disabled="pendingTool === capability.name"
                @click="emit('govern', 'deprecate', capability.name)"
              >
                Deprecate
              </button>
              <span v-if="pendingTool === capability.name" class="status-pill">pending</span>
            </div>
          </td>
        </tr>
      </tbody>
    </table>
  </div>
</template>

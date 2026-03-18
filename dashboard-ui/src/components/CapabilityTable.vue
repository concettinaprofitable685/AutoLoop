<script setup lang="ts">
import type { CapabilityRecord } from "../types";

defineProps<{
  capabilities: CapabilityRecord[];
}>();
</script>

<template>
  <section class="panel">
    <div class="panel-header">
      <h2>Capability Governance</h2>
      <span>{{ capabilities.length }} surfaces</span>
    </div>
    <div class="table-shell">
      <table class="capability-table">
        <thead>
          <tr>
            <th>Name</th>
            <th>Status</th>
            <th>Approval</th>
            <th>Health</th>
            <th>Scope</th>
            <th>Risk</th>
          </tr>
        </thead>
        <tbody>
          <tr v-for="capability in capabilities" :key="capability.name">
            <td>{{ capability.name }}</td>
            <td><span class="pill">{{ capability.status }}</span></td>
            <td>{{ capability.approval }}</td>
            <td>{{ capability.health.toFixed(2) }}</td>
            <td>{{ capability.scope }}</td>
            <td>{{ capability.risk }}</td>
          </tr>
        </tbody>
      </table>
    </div>
  </section>
</template>

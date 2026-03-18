<script setup lang="ts">
import type { CapabilityLifecycleEntry } from "../../types/capability";

defineProps<{
  entries: CapabilityLifecycleEntry[];
}>();
</script>

<template>
  <ul class="stack-list compact">
    <li v-for="entry in entries" :key="entry.lineage_key" class="stack-item">
      <div>
        <strong>{{ entry.tool_name }}</strong>
        <p>active v{{ entry.active_version ?? "n/a" }} | latest v{{ entry.latest_version }}</p>
      </div>
      <span>{{ entry.status_summary }}</span>
    </li>
  </ul>
</template>

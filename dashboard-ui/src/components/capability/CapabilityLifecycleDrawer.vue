<script setup lang="ts">
import { computed } from "vue";
import CapabilityVersionTree from "./CapabilityVersionTree.vue";
import type { CapabilityLifecycleEntry } from "../../types/capability";

const props = defineProps<{
  entries: CapabilityLifecycleEntry[];
}>();

const safeEntries = computed(() => props.entries ?? []);
</script>

<template>
  <div class="detail-stack">
    <div class="data-source-card">
      <strong>Capability lifecycle</strong>
      <p>{{ safeEntries.length }} tracked lineages</p>
    </div>
    <CapabilityVersionTree :entries="safeEntries" />
  </div>
</template>

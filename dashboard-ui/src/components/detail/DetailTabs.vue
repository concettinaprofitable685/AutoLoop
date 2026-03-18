<script setup lang="ts">
import { useUiPreferences } from "../../composables/useUiPreferences";

defineProps<{
  activeTab: "detail" | "evolution" | "governance";
}>();

const emit = defineEmits<{
  "update:activeTab": [value: "detail" | "evolution" | "governance"];
}>();

const { t } = useUiPreferences();
</script>

<template>
  <div class="tab-row">
    <button class="tab-button" :data-active="activeTab === 'detail'" @click="emit('update:activeTab', 'detail')">{{ t("tabDetail") }}</button>
    <button class="tab-button" :data-active="activeTab === 'evolution'" @click="emit('update:activeTab', 'evolution')">{{ t("tabEvolution") }}</button>
    <button class="tab-button" :data-active="activeTab === 'governance'" @click="emit('update:activeTab', 'governance')">{{ t("tabGovernance") }}</button>
  </div>
</template>

<script setup lang="ts">
import PanelFrame from "../common/PanelFrame.vue";
import { useUiPreferences } from "../../composables/useUiPreferences";

defineProps<{
  sessions: string[];
}>();

const { t } = useUiPreferences();
</script>

<template>
  <PanelFrame :title="t('sessionsTitle')" :subtitle="t('sessionsSubtitle')">
    <ul class="stack-list compact">
      <li v-for="session in sessions" :key="session" class="stack-item">
        <strong>{{ session }}</strong>
      </li>
    </ul>
  </PanelFrame>
</template>

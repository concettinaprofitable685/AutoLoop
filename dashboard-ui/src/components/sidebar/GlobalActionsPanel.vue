<script setup lang="ts">
import PanelFrame from "../common/PanelFrame.vue";
import { onMounted } from "vue";
import { useUiPreferences } from "../../composables/useUiPreferences";

const props = defineProps<{
  dashboardUrl: string;
  replayUrl: string;
}>();

const emit = defineEmits<{
  "dashboard-url-update": [value: string];
  "replay-url-update": [value: string];
  "load-dashboard-url": [value: string];
  "load-replay-url": [value: string];
  refresh: [];
  "clear-graph": [];
}>();

const {
  language,
  providerVendor,
  apiBaseUrl,
  defaultModel,
  apiKey,
  settingsMessage,
  vendorOptions,
  applyVendorPreset,
  resetToPreset,
  loadRemote,
  saveRemote,
  t
} = useUiPreferences();

onMounted(async () => {
  await loadRemote(props.dashboardUrl);
});
</script>

<template>
  <PanelFrame :title="t('globalTitle')" :subtitle="t('globalSubtitle')">
    <div class="form-stack">
      <label class="field-label">
        {{ t("dashboardUrl") }}
        <input
          :value="dashboardUrl"
          type="url"
          @input="emit('dashboard-url-update', ($event.target as HTMLInputElement).value)"
        />
      </label>
      <button class="action-button" @click="emit('load-dashboard-url', dashboardUrl)">{{ t("loadDashboard") }}</button>

      <label class="field-label">
        {{ t("replayUrl") }}
        <input
          :value="replayUrl"
          type="url"
          @input="emit('replay-url-update', ($event.target as HTMLInputElement).value)"
        />
      </label>
      <button class="action-button" @click="emit('load-replay-url', replayUrl)">{{ t("loadReplay") }}</button>

      <div class="settings-divider"></div>

      <label class="field-label">
        {{ t("language") }}
        <select v-model="language" class="settings-select">
          <option value="zh-CN">Chinese</option>
          <option value="en-US">English</option>
        </select>
      </label>

      <label class="field-label">
        {{ t("vendor") }}
        <select
          v-model="providerVendor"
          class="settings-select"
          @change="applyVendorPreset(providerVendor)"
        >
          <option
            v-for="option in vendorOptions"
            :key="option.value"
            :value="option.value"
          >
            {{ option.label }}
          </option>
        </select>
      </label>

      <label class="field-label">
        {{ t("apiKey") }}
        <input v-model="apiKey" type="password" autocomplete="off" />
      </label>

      <label class="field-label">
        {{ t("apiBaseUrl") }}
        <input v-model="apiBaseUrl" type="url" />
      </label>

      <label class="field-label">
        {{ t("model") }}
        <input v-model="defaultModel" type="text" />
      </label>

      <button class="action-button" @click="saveRemote(dashboardUrl)">{{ t("saveSettings") }}</button>
      <button class="action-button secondary" @click="resetToPreset()">{{ t("resetSettings") }}</button>
      <p class="summary-copy">{{ t("settingsHint") }}</p>
      <p v-if="settingsMessage" class="status-note">{{ settingsMessage }}</p>

      <button class="action-button secondary" @click="emit('refresh')">{{ t("refreshData") }}</button>
      <button class="action-button secondary" @click="emit('clear-graph')">{{ t("clearFocus") }}</button>
    </div>
  </PanelFrame>
</template>

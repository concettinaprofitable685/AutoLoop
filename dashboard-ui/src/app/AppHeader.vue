<script setup lang="ts">
import { useUiPreferences } from "../composables/useUiPreferences";

const { language, providerVendor, t } = useUiPreferences();
</script>

<template>
  <header class="app-header">
    <div>
      <p class="eyebrow">{{ t("headerEyebrow") }}</p>
      <h1>{{ t("headerTitle") }}</h1>
    </div>
    <div class="header-meta">
      <span>{{ t("headerMetaRuntime") }}</span>
      <span>{{ t("headerMetaGovernance") }}</span>
      <span>{{ t("headerMetaReplay") }}</span>
      <span>{{ t("headerMetaVendor") }}: {{ providerVendor }}</span>
      <label class="header-select">
        <select v-model="language">
          <option value="zh-CN">Chinese</option>
          <option value="en-US">English</option>
        </select>
      </label>
    </div>
  </header>
</template>

<script setup lang="ts">
import AppHeader from "./AppHeader.vue";
import AppLayout from "./AppLayout.vue";
import ControlPlanePage from "../pages/ControlPlanePage.vue";
</script>

<template>
  <div class="app-shell">
    <AppHeader />
    <AppLayout>
      <ControlPlanePage />
    </AppLayout>
  </div>
</template>

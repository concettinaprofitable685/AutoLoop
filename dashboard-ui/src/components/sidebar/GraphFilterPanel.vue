<script setup lang="ts">
import PanelFrame from "../common/PanelFrame.vue";
import { useUiPreferences } from "../../composables/useUiPreferences";

defineProps<{
  entityFilters: string[];
  relationFilters: string[];
  selectedEntityFilters: string[];
  selectedRelationFilters: string[];
}>();

const emit = defineEmits<{
  toggleEntityFilter: [kind: string];
  toggleRelationFilter: [kind: string];
}>();

const { t } = useUiPreferences();
</script>

<template>
  <PanelFrame :title="t('filtersTitle')" :subtitle="t('filtersSubtitle')">
    <div class="filter-group">
      <p>Entity types</p>
      <div class="chip-row">
        <button
          v-for="kind in entityFilters"
          :key="kind"
          class="chip"
          :data-active="selectedEntityFilters.includes(kind)"
          @click="emit('toggleEntityFilter', kind)"
        >
          {{ kind }}
        </button>
      </div>
    </div>
    <div class="filter-group">
      <p>Relation types</p>
      <div class="chip-row">
        <button
          v-for="kind in relationFilters"
          :key="kind"
          class="chip"
          :data-active="selectedRelationFilters.includes(kind)"
          @click="emit('toggleRelationFilter', kind)"
        >
          {{ kind }}
        </button>
      </div>
    </div>
  </PanelFrame>
</template>

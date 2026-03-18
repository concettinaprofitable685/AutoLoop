<script setup lang="ts">
defineProps<{
  title: string;
  subtitle?: string;
}>();
</script>

<template>
  <section class="panel">
    <div class="panel-header">
      <div>
        <h2>{{ title }}</h2>
        <span v-if="subtitle">{{ subtitle }}</span>
      </div>
      <slot name="actions" />
    </div>
    <slot />
  </section>
</template>

<script setup lang="ts">
import { computed, onBeforeUnmount, ref, watch } from "vue";
import ReplayEventCard from "./ReplayEventCard.vue";

const props = defineProps<{
  events: Array<Record<string, unknown>>;
  selectedIndex?: number;
}>();

const emit = defineEmits<{
  selectEvent: [payload: { index: number; tool?: string }];
}>();

const isPlaying = ref(false);
let timer: number | null = null;
const safeEvents = computed(() => props.events ?? []);

function stopPlayback() {
  if (timer !== null) {
    window.clearInterval(timer);
    timer = null;
  }
  isPlaying.value = false;
}

function emitSelection(index: number) {
  emit("selectEvent", {
    index,
    tool: safeEvents.value[index]?.tool ? String(safeEvents.value[index].tool) : undefined
  });
}

function startPlayback() {
  if (safeEvents.value.length === 0) return;
  stopPlayback();
  isPlaying.value = true;
  let cursor = typeof props.selectedIndex === "number" ? props.selectedIndex : -1;
  timer = window.setInterval(() => {
    cursor += 1;
    if (cursor >= safeEvents.value.length) {
      stopPlayback();
      return;
    }
    emitSelection(cursor);
  }, 1200);
}

function selectPrevious() {
  if (!safeEvents.value.length) return;
  const cursor = Math.max(0, (props.selectedIndex ?? 0) - 1);
  emitSelection(cursor);
}

function selectNext() {
  if (!safeEvents.value.length) return;
  const cursor = Math.min(safeEvents.value.length - 1, (props.selectedIndex ?? -1) + 1);
  emitSelection(cursor);
}

watch(
  () => props.selectedIndex,
  (value) => {
    if (typeof value === "number" && value >= safeEvents.value.length) {
      stopPlayback();
    }
  }
);

onBeforeUnmount(stopPlayback);
</script>

<template>
  <div class="timeline-list">
    <div class="chip-row">
      <button class="action-button secondary" @click="selectPrevious">Prev</button>
      <button v-if="!isPlaying" class="action-button secondary" @click="startPlayback">Play</button>
      <button v-else class="action-button secondary" @click="stopPlayback">Pause</button>
      <button class="action-button secondary" @click="selectNext">Next</button>
    </div>
    <ReplayEventCard
      v-for="(event, index) in safeEvents"
      :key="index"
      :index="index"
      :title="String(event.tool ?? event.phase ?? `event-${index + 1}`)"
      :payload="event"
      :selected="props.selectedIndex === index"
      @select-event="emit('selectEvent', $event)"
    />
  </div>
</template>

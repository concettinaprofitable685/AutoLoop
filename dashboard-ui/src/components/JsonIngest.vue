<script setup lang="ts">
import { onBeforeUnmount, ref } from "vue";

const emit = defineEmits<{
  load: [payload: string];
}>();

const remoteUrl = ref("");
const pollEnabled = ref(false);
let pollHandle: number | undefined;

function onChange(event: Event) {
  const target = event.target as HTMLInputElement;
  const file = target.files?.[0];
  if (!file) return;
  file.text().then((payload) => emit("load", payload));
}

async function loadFromUrl() {
  if (!remoteUrl.value.trim()) return;
  const response = await fetch(remoteUrl.value);
  const payload = await response.text();
  emit("load", payload);
}

function togglePolling() {
  pollEnabled.value = !pollEnabled.value;
  if (!pollEnabled.value) {
    if (pollHandle) window.clearInterval(pollHandle);
    pollHandle = undefined;
    return;
  }
  loadFromUrl();
  pollHandle = window.setInterval(loadFromUrl, 5000);
}

onBeforeUnmount(() => {
  if (pollHandle) window.clearInterval(pollHandle);
});
</script>

<template>
  <section class="panel ingest-panel">
    <div class="panel-header">
      <h2>Import Session Snapshot</h2>
      <span>JSON export</span>
    </div>
    <label class="upload-shell">
      <input type="file" accept="application/json" @change="onChange" />
      <span>Load `focus status` or exported dashboard JSON</span>
    </label>
    <div class="remote-shell">
      <input v-model="remoteUrl" type="url" placeholder="https://.../dashboard.json" />
      <button type="button" @click="loadFromUrl">Fetch remote snapshot</button>
      <button type="button" @click="togglePolling">
        {{ pollEnabled ? "Stop polling" : "Poll every 5s" }}
      </button>
    </div>
  </section>
</template>

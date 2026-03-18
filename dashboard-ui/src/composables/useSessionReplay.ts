import { ref } from "vue";
import type { SessionReplay } from "../types/replay";
import { fetchSessionReplay } from "../services/replayApi";

export function useSessionReplay(initialReplay: SessionReplay) {
  const replay = ref<SessionReplay>(initialReplay);
  const error = ref("");

  async function loadFromRaw(raw: string) {
    try {
      replay.value = JSON.parse(raw) as SessionReplay;
      error.value = "";
    } catch (loadError) {
      error.value = loadError instanceof Error ? loadError.message : "Invalid session replay";
    }
  }

  async function loadFromUrl(url: string) {
    try {
      replay.value = await fetchSessionReplay(url);
      error.value = "";
    } catch (loadError) {
      error.value = loadError instanceof Error ? loadError.message : "Failed to load session replay";
    }
  }

  return {
    replay,
    error,
    loadFromRaw,
    loadFromUrl
  };
}

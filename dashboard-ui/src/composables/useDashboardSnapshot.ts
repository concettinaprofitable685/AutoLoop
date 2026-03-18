import { ref } from "vue";
import type { DashboardSessionSnapshot } from "../types/snapshot";
import { fetchDashboardSnapshot } from "../services/snapshotApi";

export function useDashboardSnapshot(initialSnapshot: DashboardSessionSnapshot) {
  const snapshot = ref<DashboardSessionSnapshot>(initialSnapshot);
  const remoteUrl = ref("");
  const loading = ref(false);
  const error = ref("");

  async function loadFromRaw(raw: string) {
    try {
      snapshot.value = JSON.parse(raw) as DashboardSessionSnapshot;
      error.value = "";
    } catch (loadError) {
      error.value = loadError instanceof Error ? loadError.message : "Invalid dashboard snapshot";
    }
  }

  async function loadFromUrl(url: string) {
    loading.value = true;
    try {
      snapshot.value = await fetchDashboardSnapshot(url);
      remoteUrl.value = url;
      error.value = "";
    } catch (loadError) {
      error.value =
        loadError instanceof Error ? loadError.message : "Failed to load dashboard snapshot";
    } finally {
      loading.value = false;
    }
  }

  return {
    snapshot,
    remoteUrl,
    loading,
    error,
    loadFromRaw,
    loadFromUrl
  };
}

import { computed, ref } from "vue";
import type { DashboardSessionSnapshot } from "../types/snapshot";
import type { CapabilityGovernanceAction, CapabilityRecord } from "../types/capability";
import { deriveLifecycleEntries } from "../services/capabilityMapper";
import { governCapability } from "../services/governanceApi";

export function useCapabilityGovernance(snapshot: { value: DashboardSessionSnapshot }) {
  const optimisticOverrides = ref<Record<string, Partial<CapabilityRecord>>>({});
  const lifecycleEntries = computed(() =>
    deriveLifecycleEntries(snapshot.value.capabilityLifecycle)
  );
  const effectiveCapabilities = computed(() =>
    snapshot.value.capabilityCatalog.map((capability) => ({
      ...capability,
      ...(optimisticOverrides.value[capability.name] ?? {})
    }))
  );
  const pendingTool = ref("");
  const error = ref("");

  const routeForensics = computed(() => ({
    openCircuits: Object.keys(snapshot.value.runtimeCircuits ?? {}),
    failingTools: snapshot.value.verifier.failingTools,
    treatmentShare: snapshot.value.routeTreatmentShare
  }));

  function optimisticCapabilityState(
    capability: CapabilityRecord,
    action: CapabilityGovernanceAction
  ): Partial<CapabilityRecord> {
    switch (action) {
      case "verify":
        return {
          status: "active",
          approval: "verified",
          health: Math.max(capability.health, 0.85)
        };
      case "rollback":
        return {
          status: "active",
          approval: capability.approval === "pending" ? "verified" : capability.approval,
          health: Math.max(capability.health, 0.72)
        };
      case "deprecate":
        return {
          status: "deprecated",
          approval: capability.approval,
          health: Math.min(capability.health, 0.35)
        };
    }
  }

  async function govern(
    baseUrl: string,
    action: CapabilityGovernanceAction,
    tool: string
  ) {
    pendingTool.value = tool;
    const capability = snapshot.value.capabilityCatalog.find((item) => item.name === tool);
    const previous = optimisticOverrides.value[tool];
    if (capability) {
      optimisticOverrides.value = {
        ...optimisticOverrides.value,
        [tool]: optimisticCapabilityState(capability, action)
      };
    }
    try {
      await governCapability(baseUrl, action, tool);
      error.value = "";
    } catch (governError) {
      optimisticOverrides.value = previous
        ? { ...optimisticOverrides.value, [tool]: previous }
        : Object.fromEntries(
            Object.entries(optimisticOverrides.value).filter(([name]) => name !== tool)
          );
      error.value =
        governError instanceof Error ? governError.message : "Failed to govern capability";
      throw governError;
    } finally {
      pendingTool.value = "";
    }
  }

  return {
    lifecycleEntries,
    effectiveCapabilities,
    routeForensics,
    pendingTool,
    error,
    govern
  };
}

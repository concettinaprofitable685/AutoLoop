import type { CapabilityLifecycleEntry, CapabilityRecord } from "../types/capability";

export function deriveLifecycleEntries(raw: Record<string, unknown> | undefined): CapabilityLifecycleEntry[] {
  const entries = raw?.entries;
  if (!Array.isArray(entries)) return [];
  return entries as CapabilityLifecycleEntry[];
}

export function groupCapabilities(capabilities: CapabilityRecord[]) {
  return {
    active: capabilities.filter((item) => item.status === "active"),
    pending: capabilities.filter((item) => item.status.includes("pending")),
    deprecated: capabilities.filter((item) => item.status.includes("deprecated"))
  };
}

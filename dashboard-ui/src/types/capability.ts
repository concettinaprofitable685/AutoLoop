export interface CapabilityRecord {
  name: string;
  status: string;
  approval: string;
  health: number;
  scope: string;
  risk: string;
}

export interface CapabilityLifecycleEntry {
  tool_name: string;
  lineage_key: string;
  active_version?: number | null;
  latest_version: number;
  stable_version?: number | null;
  deprecated_versions: number[];
  rolled_back_versions: number[];
  average_health: number;
  status_summary: string;
}

export type CapabilityGovernanceAction = "verify" | "deprecate" | "rollback";

export type CapabilityGovernanceAction = "verify" | "deprecate" | "rollback";

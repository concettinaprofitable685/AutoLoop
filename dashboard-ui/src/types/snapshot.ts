import type { CapabilityRecord } from "./capability";
import type { SessionReplay } from "./replay";

export interface DashboardGraphLens {
  entities: number;
  relationships: number;
  communities: number;
  forgedCapabilityCount: number;
  topEntities: string[];
}

export interface DashboardVerifierLens {
  verdict: string;
  score: number;
  summary: string;
  failingTools: string[];
}

export interface DashboardSessionSnapshot {
  sessionId: string;
  anchor: string;
  ceoSummary: string;
  validationSummary: string;
  routeTreatmentShare: number;
  readiness: boolean;
  capabilityCatalog: CapabilityRecord[];
  proxyForensics: Record<string, unknown>;
  researchHealth: Record<string, unknown>;
  graph: DashboardGraphLens;
  verifier: DashboardVerifierLens;
  operationsNotes: string[];
  capabilityLifecycle?: Record<string, unknown>;
  runtimeCircuits?: Record<string, unknown>;
  replay?: SessionReplay;
}

export interface CapabilityRecord {
  name: string;
  status: "active" | "pending" | "deprecated";
  approval: "verified" | "pending" | "rejected";
  health: number;
  scope: string;
  risk: string;
}

export interface ProxyForensics {
  provider: string;
  poolSize: number;
  used: string[];
  exhausted: string[];
  openCircuit: string[];
  likelyPressure: boolean;
  warningSamples: string[];
}

export interface ResearchHealth {
  backend: string;
  liveFetchEnabled: boolean;
  curlAvailable: boolean;
  nodeAvailable: boolean;
  browserRenderConfigured: boolean;
  proxyPoolSize: number;
  browserSessionPoolSize: number;
  antiBotProfile: string;
}

export interface GraphLens {
  entities: number;
  relationships: number;
  communities: number;
  forgedCapabilityCount: number;
  topEntities: string[];
}

export interface VerifierLens {
  verdict: "pass" | "needs_iteration" | "reject";
  score: number;
  summary: string;
  failingTools: string[];
}

export interface SessionSnapshot {
  sessionId: string;
  anchor: string;
  ceoSummary: string;
  validationSummary: string;
  routeTreatmentShare: number;
  readiness: boolean;
  capabilityCatalog: CapabilityRecord[];
  proxyForensics: ProxyForensics;
  researchHealth: ResearchHealth;
  graph: GraphLens;
  verifier: VerifierLens;
  operationsNotes: string[];
  capabilityLifecycle?: {
    totalLineages: number;
    rollbackReady: number;
    deprecated: number;
  };
  runtimeCircuits?: Record<string, unknown>;
  replay?: SessionReplay;
}

export interface SessionReplay {
  sessionId: string;
  deliberation?: {
    round_count?: number;
    final_execution_order?: string[];
    consensus_signals?: string[];
  };
  executionFeedback: Array<Record<string, unknown>>;
  traces: Array<Record<string, unknown>>;
  routeAnalytics?: Record<string, unknown>;
  failureForensics?: Record<string, unknown>;
}

import type { DashboardSessionSnapshot } from "../types/snapshot";
import type { CapabilityRecord } from "../types/capability";

export const DEFAULT_SNAPSHOT_BASE = "http://127.0.0.1:8787";

export function buildDashboardUrl(sessionId: string, baseUrl = DEFAULT_SNAPSHOT_BASE): string {
  return `${baseUrl.replace(/\/$/, "")}/api/dashboard/${encodeURIComponent(sessionId)}`;
}

export function buildCatalogUrl(sessionId: string, baseUrl = DEFAULT_SNAPSHOT_BASE): string {
  return `${baseUrl.replace(/\/$/, "")}/api/catalog/${encodeURIComponent(sessionId)}`;
}

export async function fetchDashboardSnapshot(url: string): Promise<DashboardSessionSnapshot> {
  const response = await fetch(url);
  const raw = await response.text();
  return normalizeDashboardSnapshot(JSON.parse(raw) as Record<string, unknown>);
}

function normalizeDashboardSnapshot(raw: Record<string, unknown>): DashboardSessionSnapshot {
  const capabilityCatalog = Array.isArray(raw.capabilityCatalog)
    ? raw.capabilityCatalog
    : Array.isArray(raw.capability_catalog)
      ? raw.capability_catalog
      : [];
  const graphRaw = asRecord(raw.graph);
  const verifierRaw = asRecord(raw.verifier);

  return {
    sessionId: asString(raw.sessionId ?? raw.session_id),
    anchor: asString(raw.anchor),
    ceoSummary: asString(raw.ceoSummary ?? raw.ceo_summary),
    validationSummary: asString(raw.validationSummary ?? raw.validation_summary),
    routeTreatmentShare: asNumber(raw.routeTreatmentShare ?? raw.route_treatment_share),
    readiness: Boolean(raw.readiness),
    capabilityCatalog: capabilityCatalog.map(normalizeCapability),
    proxyForensics: asRecord(raw.proxyForensics ?? raw.proxy_forensics),
    researchHealth: asRecord(raw.researchHealth ?? raw.research_health),
    graph: {
      entities: asNumber(graphRaw.entities),
      relationships: asNumber(graphRaw.relationships),
      communities: asNumber(graphRaw.communities),
      forgedCapabilityCount: asNumber(
        graphRaw.forgedCapabilityCount ?? graphRaw.forged_capability_count
      ),
      topEntities: asStringArray(graphRaw.topEntities ?? graphRaw.top_entities)
    },
    verifier: {
      verdict: asString(verifierRaw.verdict),
      score: asNumber(verifierRaw.score),
      summary: asString(verifierRaw.summary),
      failingTools: asStringArray(verifierRaw.failingTools ?? verifierRaw.failing_tools)
    },
    operationsNotes: asStringArray(raw.operationsNotes ?? raw.operations_notes),
    capabilityLifecycle: asRecord(raw.capabilityLifecycle ?? raw.capability_lifecycle),
    runtimeCircuits: asRecord(raw.runtimeCircuits ?? raw.runtime_circuits)
  };
}

function normalizeCapability(raw: unknown): CapabilityRecord {
  const record = asRecord(raw);
  return {
    name: asString(record.name ?? record.tool_name),
    status: asString(record.status),
    approval: asString(record.approval ?? record.approval_status),
    health: asNumber(record.health ?? record.health_score),
    scope: asString(record.scope),
    risk: asString(record.risk)
  };
}

function asRecord(value: unknown): Record<string, unknown> {
  return value && typeof value === "object" && !Array.isArray(value)
    ? (value as Record<string, unknown>)
    : {};
}

function asString(value: unknown): string {
  return typeof value === "string" ? value : "";
}

function asNumber(value: unknown): number {
  return typeof value === "number" ? value : 0;
}

function asStringArray(value: unknown): string[] {
  return Array.isArray(value) ? value.map((item) => String(item)) : [];
}

import type { SessionReplay } from "../types/replay";

export function buildReplayUrl(sessionId: string, baseUrl = "http://127.0.0.1:8787"): string {
  return `${baseUrl.replace(/\/$/, "")}/api/replay/${encodeURIComponent(sessionId)}`;
}

export async function fetchSessionReplay(url: string): Promise<SessionReplay> {
  const response = await fetch(url);
  const raw = await response.text();
  return normalizeSessionReplay(JSON.parse(raw) as Record<string, unknown>);
}

function normalizeSessionReplay(raw: Record<string, unknown>): SessionReplay {
  const deliberation = asRecord(raw.deliberation);
  return {
    sessionId: asString(raw.sessionId ?? raw.session_id),
    deliberation: Object.keys(deliberation).length
      ? {
          round_count: asNumber(deliberation.round_count),
          final_execution_order: asStringArray(deliberation.final_execution_order),
          consensus_signals: asStringArray(deliberation.consensus_signals),
          rounds: Array.isArray(deliberation.rounds)
            ? (deliberation.rounds as Array<Record<string, unknown>>)
            : []
        }
      : undefined,
    executionFeedback: Array.isArray(raw.executionFeedback)
      ? raw.executionFeedback
      : Array.isArray(raw.execution_feedback)
        ? raw.execution_feedback
        : [],
    traces: Array.isArray(raw.traces) ? raw.traces.map((item) => normalizeTrace(item)) : [],
    routeAnalytics: asRecord(raw.routeAnalytics ?? raw.route_analytics),
    failureForensics: asRecord(raw.failureForensics ?? raw.failure_forensics)
  };
}

function normalizeTrace(raw: unknown) {
  const record = asRecord(raw);
  return {
    span_name: asString(record.span_name),
    level: asString(record.level),
    detail: asString(record.detail),
    created_at_ms:
      typeof record.created_at_ms === "number" ? record.created_at_ms : undefined
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

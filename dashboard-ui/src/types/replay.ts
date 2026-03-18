export interface ReplayTrace {
  span_name: string;
  level: string;
  detail: string;
  created_at_ms?: number;
}

export interface SessionReplay {
  sessionId: string;
  deliberation?: {
    round_count?: number;
    final_execution_order?: string[];
    consensus_signals?: string[];
    rounds?: Array<Record<string, unknown>>;
  };
  executionFeedback: Array<Record<string, unknown>>;
  traces: ReplayTrace[];
  routeAnalytics?: Record<string, unknown>;
  failureForensics?: Record<string, unknown>;
}

export interface ReplaySelection {
  index?: number;
  tool?: string;
  trace?: string;
}

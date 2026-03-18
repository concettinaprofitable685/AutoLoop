export interface UiGraphNode {
  id: string;
  label: string;
  kind: "anchor" | "agent" | "capability" | "entity" | "verifier" | "runtime";
  status?: string;
  x: number;
  y: number;
  source?: string;
  metadata?: Record<string, unknown>;
}

export interface UiGraphEdge {
  id: string;
  source: string;
  target: string;
  kind: "references" | "evolves" | "verifies" | "routes";
}

export interface UiGraphModel {
  nodes: UiGraphNode[];
  edges: UiGraphEdge[];
}

import type { AgentRecord } from "../types/agent";
import type { UiGraphEdge, UiGraphModel, UiGraphNode } from "../types/graph";
import type { SessionReplay } from "../types/replay";
import type { DashboardSessionSnapshot } from "../types/snapshot";

export function deriveAgents(
  snapshot: DashboardSessionSnapshot,
  replay: SessionReplay | null
): AgentRecord[] {
  const roundSpeakers =
    replay?.deliberation?.rounds?.map((round) => String(round.speaker ?? "agent")) ?? [];
  const uniqueAgents = Array.from(
    new Set(["ceo-agent", "execution-agent", "knowledge-agent", "cli-agent", ...roundSpeakers])
  );
  return uniqueAgents.map((name, index) => ({
    id: name,
    name,
    status:
      name.includes("execution")
        ? "running"
        : name.includes("judge")
          ? "evolving"
          : index % 3 === 0
            ? "sleeping"
            : "running",
    reputation: Number((0.58 + index * 0.07).toFixed(2)),
    phase: name.includes("judge")
      ? "verification"
      : name.includes("execution")
        ? "execution"
        : "reasoning"
  }));
}

export function mapSnapshotToGraph(
  snapshot: DashboardSessionSnapshot,
  replay: SessionReplay | null
): UiGraphModel {
  const nodes: UiGraphNode[] = [];
  const edges: UiGraphEdge[] = [];

  nodes.push({
    id: "anchor",
    label: snapshot.anchor,
    kind: "anchor",
    status: snapshot.readiness ? "ready" : "iteration",
    x: 120,
    y: 220,
    source: "dashboard snapshot"
  });

  snapshot.graph.topEntities.forEach((entity, index) => {
    const id = `entity:${index}`;
    nodes.push({
      id,
      label: entity,
      kind: "entity",
      x: 320 + (index % 3) * 150,
      y: 100 + Math.floor(index / 3) * 110,
      source: "graph snapshot"
    });
    edges.push({
      id: `edge:anchor:${id}`,
      source: "anchor",
      target: id,
      kind: "references"
    });
  });

  snapshot.capabilityCatalog.forEach((capability, index) => {
    const id = `capability:${index}`;
    nodes.push({
      id,
      label: capability.name,
      kind: "capability",
      status: capability.status,
      x: 260 + (index % 2) * 230,
      y: 360 + Math.floor(index / 2) * 110,
      source: "capability catalog",
      metadata: capability as Record<string, unknown>
    });
    edges.push({
      id: `edge:anchor:capability:${index}`,
      source: "anchor",
      target: id,
      kind: "evolves"
    });
  });

  deriveAgents(snapshot, replay).forEach((agent, index) => {
    const id = `agent:${agent.id}`;
    nodes.push({
      id,
      label: agent.name,
      kind: "agent",
      status: agent.status,
      x: 700,
      y: 90 + index * 90,
      source: "session replay",
      metadata: agent as Record<string, unknown>
    });
    edges.push({
      id: `edge:agent:${agent.id}:anchor`,
      source: id,
      target: "anchor",
      kind: "routes"
    });
  });

  nodes.push({
    id: "verifier",
    label: `Verifier ${snapshot.verifier.verdict}`,
    kind: "verifier",
    status: snapshot.verifier.verdict,
    x: 920,
    y: 220,
    source: "verifier report",
    metadata: snapshot.verifier as Record<string, unknown>
  });
  edges.push({
    id: "edge:verifier:anchor",
    source: "verifier",
    target: "anchor",
    kind: "verifies"
  });

  nodes.push({
    id: "runtime",
    label: "Runtime Circuits",
    kind: "runtime",
    status: Object.keys(snapshot.runtimeCircuits ?? {}).length > 0 ? "active" : "idle",
    x: 930,
    y: 400,
    source: "runtime circuits",
    metadata: snapshot.runtimeCircuits ?? {}
  });
  edges.push({
    id: "edge:runtime:verifier",
    source: "runtime",
    target: "verifier",
    kind: "verifies"
  });

  return { nodes, edges };
}

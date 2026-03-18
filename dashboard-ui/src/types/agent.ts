export interface AgentRecord {
  id: string;
  name: string;
  status: "running" | "evolving" | "sleeping" | "error";
  reputation: number;
  phase: string;
}

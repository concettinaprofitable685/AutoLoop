import type { CapabilityGovernanceAction } from "../types/capability";

export async function governCapability(
  baseUrl: string,
  action: CapabilityGovernanceAction,
  tool: string
): Promise<Record<string, unknown>> {
  const response = await fetch(`${baseUrl.replace(/\/$/, "")}/api/capabilities/govern`, {
    method: "POST",
    headers: {
      "Content-Type": "application/json"
    },
    body: JSON.stringify({ action, tool })
  });
  const raw = await response.text();
  return JSON.parse(raw) as Record<string, unknown>;
}

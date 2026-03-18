# AutoLoop Architecture

This document is a concise map of the current system.

## Runtime Flow

1. CLI receives intent (`main.rs`).
2. `AutoLoopApp` assembles runtime subsystems (`src/lib.rs`).
3. Orchestration drives requirement -> CEO -> swarm -> execution (`src/orchestration/mod.rs`).
4. Runtime guard/verifier enforce bounded execution (`src/runtime/mod.rs`).
5. Knowledge and learning artifacts are persisted through SpacetimeDB adapter.

## Core Modules

- `src/orchestration/`
  - Swarm planning, debate rounds, route selection, validation.
- `src/runtime/`
  - Runtime guard, circuit breaker state, evaluation and verifier logic.
- `src/providers/`
  - OpenAI-compatible HTTP provider abstraction and model routing.
- `src/tools/`
  - Tool registry, forged MCP capability governance.
- `src/research/`
  - Research execution backends and anchor-driven data acquisition.
- `src/rag/`
  - GraphRAG snapshot/update/retrieval and graph signals.
- `src/memory/`
  - Learning assets and memory retrieval/consolidation.
- `src/observability/`
  - Route analytics, failure forensics, dashboard artifacts.
- `src/dashboard_server.rs`
  - Minimal HTTP + SSE backend for dashboard snapshots/replay/governance.

## Data and Storage

- Primary runtime record layer: SpacetimeDB adapter (`autoloop-spacetimedb-adapter`).
- SpacetimeDB module crate: `spacetimedb/`.
- Runtime dashboard/replay artifacts: `deploy/runtime/` (local operational outputs).

## Frontend Control Plane

- Location: `dashboard-ui/`
- Stack: Vue 3 + TypeScript + Vite
- Features:
  - Capability governance actions
  - Session replay
  - Graph canvas overlays
  - SSE event updates
  - Operator settings (language/vendor/base URL/model/API key)

## Deployment Surfaces

- Local scripts and templates: `deploy/`
- K8s manifests: `deploy/k8s/`
- Monitoring templates: `deploy/monitoring/`
- One-command startup scripts:
  - `deploy/scripts/start-autoloop.ps1`
  - `deploy/scripts/start-autoloop.sh`


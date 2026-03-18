# AutoLoop

[![CI](https://github.com/OWNER/REPO/actions/workflows/ci.yml/badge.svg)](https://github.com/OWNER/REPO/actions/workflows/ci.yml)
[![Tests](https://img.shields.io/badge/tests-56%20passed-brightgreen)](https://github.com/OWNER/REPO/actions/workflows/ci.yml)
[![Release](https://img.shields.io/badge/release-v0.1.0--alpha-blue)](https://github.com/OWNER/REPO/releases/tag/v0.1.0-alpha)

AutoLoop is a Rust autonomous runtime built on SpacetimeDB.

It combines:

- requirement clarification
- CEO-led swarm orchestration
- forged MCP capability management
- GraphRAG and learning memory
- verifier-gated execution
- observability and deployment assets
- a Vue 3 + TypeScript + Vite operations dashboard

The goal is to turn a user request into a governed execution loop:

`requirement-agent -> CEO -> planner/critic/judge -> capability catalog -> execution-agent -> verifier -> learning -> observability`

## Release

- Current planned release: `v0.1.0-alpha`
- Release notes: [RELEASE_NOTES_v0.1.0-alpha.md](/D:/AutoLoop/autoloop-app/RELEASE_NOTES_v0.1.0-alpha.md)
- Architecture guide: [ARCHITECTURE.md](/D:/AutoLoop/autoloop-app/ARCHITECTURE.md)
- API summary: [API.md](/D:/AutoLoop/autoloop-app/API.md)
- Issue backlog for public tracking: [ISSUE_BACKLOG_v0.1.0-alpha.md](/D:/AutoLoop/autoloop-app/docs/ISSUE_BACKLOG_v0.1.0-alpha.md)

`OWNER/REPO` placeholders above should be replaced after remote repository binding.

## 5-Minute E2E Demo

- Windows demo script: [e2e-5min.ps1](/D:/AutoLoop/autoloop-app/demo/e2e-5min.ps1)
- Linux demo script: [e2e-5min.sh](/D:/AutoLoop/autoloop-app/demo/e2e-5min.sh)
- Recording checklist: [RECORDING_CHECKLIST.md](/D:/AutoLoop/autoloop-app/demo/RECORDING_CHECKLIST.md)

## Status

This repository is now in a reasonable state to open source as an engineering prototype.

That means:

- the core architecture is implemented
- the main runtime path compiles and tests pass
- the repo already includes config, deployment, and operational scaffolding

It does **not** mean:

- all behaviors are production-hardened
- all MCP integrations are fully real-world complete
- the verifier, GraphRAG, and learning logic are final

Current validation:

- `cargo check --workspace` passes
- `cargo test --workspace` passes

## What It Does

AutoLoop currently supports:

- multi-turn requirement clarification with frozen scope and confirmation signals
- CEO-driven swarm planning
- planner / critic / judge deliberation artifacts
- forged MCP capability catalog with governance
- execution constrained to `active + verified` catalog capabilities
- runtime guard checks for risk, approval, and bounded execution
- GraphRAG snapshot generation and incremental merge
- task-to-capability graph mapping
- learning consolidation from episodes, witness logs, causal edges, and skills
- verifier gating with task judgement, route correctness, and capability regression
- observability records for route analytics, failure forensics, dashboard snapshots, and operations reports
- deployment assets for local containers and Kubernetes

## Architecture

### Core flow

1. `requirement-agent` clarifies the request and freezes scope.
2. `CEO` creates the high-level route.
3. `planner / critic / judge` produce a bounded deliberation artifact.
4. `cli-agent` forges or updates MCP capabilities.
5. `execution-agent` selects only from the governed capability catalog.
6. `GraphRAG` stores graph state and capability edges.
7. `learning` consolidates outcomes into reusable evidence.
8. `verifier` decides pass / needs-iteration / reject.
9. `observability` records how and why the system behaved.

### Workspace layout

- `src/`
  - root application, orchestration, runtime, learning, GraphRAG, tools, providers
- `src/module_bindings/`
  - generated Rust bindings from SpacetimeDB CLI
- `autoloop-spacetimedb-adapter/`
  - storage adapter and repository isolation layer
- `spacetimedb/`
  - server-side SpacetimeDB module crate
- `deploy/`
  - config, backup/restore, and deployment templates
- `dashboard-ui/`
  - Vue 3 + TypeScript + Vite dashboard for operations, governance, and research health

## Important Modules

- `src/orchestration/mod.rs`
  - requirement flow, CEO/swarm routing, execution routing, validation
- `src/runtime/mod.rs`
  - runtime guard, immutable evaluation, verifier, capability regression
- `src/tools/mod.rs`
  - tool registry, forged capability catalog, governance actions
- `src/tools/cli_forge.rs`
  - MCP capability forging and catalog mutation tools
- `src/rag/mod.rs`
  - GraphRAG updates, forged capability graph surfaces, incremental merge
- `src/memory/mod.rs`
  - memory retrieval, learning persistence, consolidation
- `src/observability/mod.rs`
  - route analytics, failure forensics, dashboard and operations reports
- `src/lib.rs`
  - application assembly and end-to-end persistence glue

## SpacetimeDB Model

SpacetimeDB is the primary system-of-record for:

- schedule events
- permissions
- agent state
- knowledge records
- forged capability manifests
- learning assets
- verifier outputs
- observability outputs

The root app uses the official Rust client pattern:

- depends on `spacetimedb-sdk`
- reserves `src/module_bindings/` for generated bindings
- uses `build.rs` to generate bindings when the `spacetime` CLI is available

## Observability

AutoLoop persists operational records into SpacetimeDB under keys such as:

- `observability:{session}:route-analytics`
- `observability:{session}:failure-forensics`
- `observability:{session}:dashboard`
- `observability:{session}:operations-report`
- `observability:{session}:trace:*`

These records explain:

- why a route was selected
- which tools or capabilities degraded
- whether runtime guards blocked or gated execution
- what the verifier concluded

Research and long-horizon memory also persist:

- `research:{session}:proxy-forensics`
- `research:{session}:report`
- `research:{session}:follow-up-status`
- `graph:global:snapshot`

## Dashboard UI

The repository now includes a frontend at [dashboard-ui](/D:/AutoLoop/autoloop-app/dashboard-ui).

It is built with:

- Vue 3
- TypeScript
- Vite

The dashboard is designed around the runtime already exposed by AutoLoop:

- verifier score and readiness
- capability governance health
- route treatment share
- research backend health
- proxy pressure and failure forensics
- graph memory and global graph snapshot summaries

See [dashboard-ui/README.md](/D:/AutoLoop/autoloop-app/dashboard-ui/README.md) for local run and build steps.

## Quick Start

### Prerequisites

- Rust toolchain
- optional: SpacetimeDB CLI
- optional: Docker / Docker Compose

### Local run

```powershell
cargo run --manifest-path D:\AutoLoop\autoloop-app\Cargo.toml -- --message "Build a swarm that uses graph memory and MCP execution" --swarm
```

### Local checks

```powershell
cargo check --workspace --manifest-path D:\AutoLoop\autoloop-app\Cargo.toml
cargo test --workspace --manifest-path D:\AutoLoop\autoloop-app\Cargo.toml
```

### Browser research runtime

Supported real research backends:

- `browser_fetch`
  - use a Browserless-style render endpoint
- `playwright_cli`
  - use local `node + playwright` for true browser rendering
- `firecrawl`
  - use Firecrawl search/scrape APIs

Recommended health checks:

```powershell
cargo run --manifest-path D:\AutoLoop\autoloop-app\Cargo.toml -- system health
cargo run --manifest-path D:\AutoLoop\autoloop-app\Cargo.toml -- crawl status --anchor-id cli:focus
```

Recommended config knobs:

- `research.browser_render_url`
- `research.playwright_node_binary`
- `research.browser_session_pool`
- `research.proxy_pool`
- `research.anti_bot_profile`
- `research.rotate_proxy_per_request`

For local Playwright execution:

```powershell
node --version
npx playwright install chromium
```

Deployment assets included:

- [browserless-deployment.yaml](/D:/AutoLoop/autoloop-app/deploy/k8s/browserless-deployment.yaml)
- [playwright-worker-deployment.yaml](/D:/AutoLoop/autoloop-app/deploy/k8s/playwright-worker-deployment.yaml)
- [browserless-secret-template.yaml](/D:/AutoLoop/autoloop-app/deploy/k8s/browserless-secret-template.yaml)
- [autoloop-external-secret.yaml](/D:/AutoLoop/autoloop-app/deploy/k8s/autoloop-external-secret.yaml)

`docker-compose.yml` now includes a local `browserless` service for external render execution.

### Config files

- `deploy/config/autoloop.dev.toml`
- `deploy/config/autoloop.prod.toml`

Use dev config locally and prod config for container or cluster deployment.

## Deployment

Included assets:

- `Dockerfile`
- `docker-compose.yml`
- `deploy/k8s/autoloop-deployment.yaml`
- `deploy/k8s/autoloop-secret-template.yaml`
- `deploy/k8s/autoloop-external-secret.yaml`
- `deploy/k8s/autoloop-servicemonitor.yaml`
- `deploy/k8s/browserless-deployment.yaml`
- `deploy/k8s/playwright-worker-deployment.yaml`
- `deploy/backup/backup.ps1`
- `deploy/backup/restore.ps1`
- `deploy/monitoring/prometheus.yml`
- `deploy/monitoring/prometheus-rules.yaml`
- `deploy/monitoring/alertmanager-config.yaml`

These are production-oriented templates, not a complete platform stack.

You will likely still want to add:

- real secret manager wiring
- persistent volume strategy
- CI/CD
- ingress / service manifests
- monitoring backend
- frontend deployment for `dashboard-ui`

## Open Source Readiness Checklist

Before publishing, I strongly recommend checking these:

- confirm there are no private credentials, internal URLs, or local-only assumptions in config files
- decide on final license and add a `LICENSE` file if needed
- add issue / PR templates if you want community contributions
- decide whether generated bindings should be committed or regenerated in CI
- add a short roadmap so users understand what is prototype vs stable

## Current Gaps

This repo is strong enough to publish, but these parts are still evolving:

- production-grade MCP interoperability
- deeper verifier policies and regression coverage
- richer dashboard / UI layer
- full backup/export integration with real SpacetimeDB operational tooling
- stronger production isolation and runtime recovery

## Recommended Positioning

If you open source now, I’d position it as:

> A Rust + SpacetimeDB autonomous runtime prototype for governed agent execution, capability catalog management, GraphRAG-backed memory, and verifier-driven swarm orchestration.

That framing is honest and strong.

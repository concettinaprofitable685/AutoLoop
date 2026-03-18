# AutoLoop v0.1.0-alpha

Release type: alpha (engineering prototype)

## Highlights

- Rust runtime with governed swarm orchestration.
- OpenAI-compatible provider path with environment-injected API key.
- GraphRAG + learning memory + verifier loop.
- Dashboard backend + Vue 3 control plane.
- Capability governance actions (`verify`, `deprecate`, `rollback`).
- Local startup scripts for Windows and Linux.

## Implemented

- Requirement -> CEO -> swarm -> execution -> verifier flow.
- Runtime guard + circuit breaker model.
- Capability catalog and forged capability lifecycle.
- Dashboard snapshots/replay and SSE notifications.
- Basic operator settings API for language/vendor/model/base-url/api-key.
- CI pipeline for Rust and frontend build.

## Not Implemented / Partial

- Full production hardening for all MCP providers.
- Rich realtime charting and full replay animation timeline.
- Complete browser anti-bot strategy across all providers.
- Multi-tenant authn/authz and policy admin panel.

## Validation Snapshot

- `cargo check --workspace` passed.
- `cargo test --workspace` passed.
- `npx vite build` passed.

## Upgrade Notes

- Runtime/operator settings are persisted in `deploy/runtime/operator-settings.json`.
- Do not commit runtime artifacts or secrets.


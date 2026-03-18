# Contributing to AutoLoop

Thanks for helping improve AutoLoop.

## Development Setup

1. Install Rust (stable).
2. Install Node.js 20+.
3. Install dashboard dependencies:
   - `cd dashboard-ui`
   - `npm install`

## Quick Validation

Run these before opening a PR:

1. `cargo check --workspace --manifest-path ./Cargo.toml`
2. `cargo test --workspace --manifest-path ./Cargo.toml`
3. `cd dashboard-ui && npx vite build`

## Coding Expectations

- Keep changes focused and minimal.
- Prefer small, composable modules over large edits.
- Preserve existing naming/style patterns in touched files.
- Add tests for behavior changes when practical.

## Commit Message Convention

Use conventional, scoped commit messages:

- `feat: ...`
- `fix: ...`
- `docs: ...`
- `chore: ...`

Examples:

- `feat(provider): add openai-compatible outbound tool schema`
- `fix(dashboard): avoid reactive ref unwrap errors in templates`
- `docs: add architecture and release notes for v0.1.0-alpha`
- `chore(ci): add frontend build stage`

## Pull Request Checklist

1. Explain what changed and why.
2. Link related issues or context.
3. Include validation output (check/test/build).
4. Note any follow-up work not included in this PR.

## Security Notes

- Never commit real API keys or secrets.
- Runtime/operator settings should stay in runtime artifacts, not repo config.
- Keep `.gitignore` exclusions intact for `target`, `node_modules`, and `deploy/runtime`.

# Issue Backlog for v0.1.0-alpha

This backlog is ready to be published as public GitHub issues.

## 1. Provider outbound tool-calling schema
- Type: `feat`
- Module: `src/providers/mod.rs`
- Goal: Send tool schema in OpenAI-compatible requests (not only parse tool_calls on response).
- Acceptance:
1. Request body includes tool definitions.
2. Tool-call roundtrip test added.
3. Existing provider tests remain green.

## 2. Capability governance version tree UI
- Type: `feat`
- Module: `dashboard-ui/src/components/capability`
- Goal: Visualize lineage as explicit version tree with active/stable/deprecated markers.
- Acceptance:
1. Tree view supports expand/collapse.
2. Current active version is highlighted.
3. Rollback target can be selected from tree.

## 3. Runtime guard policy presets
- Type: `feat`
- Module: `src/runtime/mod.rs`, `src/config/mod.rs`
- Goal: Add strict/balanced/permissive runtime guard presets.
- Acceptance:
1. Presets selectable from config.
2. Guard report shows applied preset.
3. Default stays backward compatible.

## 4. Research source trust scoring upgrade
- Type: `feat`
- Module: `src/research/mod.rs`
- Goal: Improve source trust score with domain weighting and verifier feedback.
- Acceptance:
1. Domain trust profile file supported.
2. Trust score exposed in persisted research artifact.
3. Sorting uses trust score as weighted factor.

## 5. GraphRAG alias normalization quality pass
- Type: `feat`
- Module: `src/rag/mod.rs`, `src/rag/retrieval.rs`
- Goal: Improve alias matching for multilingual and punctuation variants.
- Acceptance:
1. Alias normalization utilities with tests.
2. Query-aware rerank uses normalized aliases.
3. Cross-session merge quality test added.

## 6. Session replay step animation
- Type: `feat`
- Module: `dashboard-ui/src/components/replay`, `dashboard-ui/src/components/canvas`
- Goal: Replay timeline drives edge/node transitions step-by-step.
- Acceptance:
1. Play/pause/seek deterministic behavior.
2. Node/edge transition classes for each step.
3. Replay does not break selection state.

## 7. Dashboard capability governance toasts
- Type: `fix`
- Module: `dashboard-ui/src/components/sidebar/GlobalActionsPanel.vue`, composables
- Goal: Add user-visible success/error feedback for verify/deprecate/rollback.
- Acceptance:
1. Success toast after mutation + SSE sync.
2. Error toast on mutation failure and rollback.
3. Pending state always cleared.

## 8. Observability export endpoint
- Type: `feat`
- Module: `src/dashboard_server.rs`
- Goal: Add API endpoint to export consolidated observability report by session.
- Acceptance:
1. `GET /api/observability/:session` implemented.
2. Route analytics + forensics + verifier included.
3. Error handling returns structured JSON.

## 9. One-command startup health check mode
- Type: `chore`
- Module: `deploy/scripts/start-autoloop.ps1`, `deploy/scripts/start-autoloop.sh`
- Goal: Add optional health-check flag and fail fast output.
- Acceptance:
1. Script supports health-check-only mode.
2. Non-zero exit on failed backend/frontend probe.
3. Logs path printed on failure.

## 10. CI cache and lint stage
- Type: `chore`
- Module: `.github/workflows/ci.yml`
- Goal: Add lint stage and optimize cache usage.
- Acceptance:
1. Rust fmt/clippy stage added.
2. Frontend lint/ts-check stage added.
3. Workflow time reduced with cache reuse.

## 11. API settings endpoint secret masking
- Type: `security`
- Module: `src/dashboard_server.rs`
- Goal: Mask API key in `GET /api/operator/settings` while keeping save behavior.
- Acceptance:
1. GET endpoint returns masked key.
2. POST still accepts full key.
3. UI supports save without forcing visible plain key.

## 12. Public quickstart e2e validation script
- Type: `docs`
- Module: `demo/`
- Goal: Keep 5-minute end-to-end demo script stable for external reviewers.
- Acceptance:
1. Script runs from clean checkout.
2. Produces deterministic session outputs.
3. README links demo steps and expected output.


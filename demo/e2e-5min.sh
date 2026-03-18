#!/usr/bin/env bash
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
CONFIG_PATH="${REPO_ROOT}/deploy/config/autoloop.dev.toml"
SESSION="demo-5min"

if [[ -z "${OPENAI_API_KEY:-}" ]]; then
  echo "OPENAI_API_KEY is required for real API demo."
  exit 1
fi

echo "[1/4] Running direct API smoke..."
cargo run --manifest-path "${REPO_ROOT}/Cargo.toml" -- --config "${CONFIG_PATH}" --session "${SESSION}-direct" --message "Reply with exactly: e2e-ok"

echo "[2/4] Running swarm flow..."
cargo run --manifest-path "${REPO_ROOT}/Cargo.toml" -- --config "${CONFIG_PATH}" --session "${SESSION}" --swarm --message "Summarize Rust reliability in three bullet points."

echo "[3/4] Exporting dashboard snapshot..."
cargo run --manifest-path "${REPO_ROOT}/Cargo.toml" -- --config "${CONFIG_PATH}" system dashboard --session "${SESSION}"

echo "[4/4] Exporting replay..."
cargo run --manifest-path "${REPO_ROOT}/Cargo.toml" -- --config "${CONFIG_PATH}" system replay --session "${SESSION}"

echo "Demo complete. Session: ${SESSION}"


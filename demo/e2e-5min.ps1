$ErrorActionPreference = "Stop"

$RepoRoot = Split-Path -Parent $PSScriptRoot
$ConfigPath = Join-Path $RepoRoot "deploy\config\autoloop.dev.toml"
$Session = "demo-5min"

if (-not $env:OPENAI_API_KEY) {
  Write-Error "OPENAI_API_KEY is required for real API demo."
}

Write-Host "[1/4] Running direct API smoke..."
cargo run --manifest-path (Join-Path $RepoRoot "Cargo.toml") -- --config $ConfigPath --session "$Session-direct" --message "Reply with exactly: e2e-ok"

Write-Host "[2/4] Running swarm flow..."
cargo run --manifest-path (Join-Path $RepoRoot "Cargo.toml") -- --config $ConfigPath --session $Session --swarm --message "Summarize Rust reliability in three bullet points."

Write-Host "[3/4] Exporting dashboard snapshot..."
cargo run --manifest-path (Join-Path $RepoRoot "Cargo.toml") -- --config $ConfigPath system dashboard --session $Session

Write-Host "[4/4] Exporting replay..."
cargo run --manifest-path (Join-Path $RepoRoot "Cargo.toml") -- --config $ConfigPath system replay --session $Session

Write-Host "Demo complete. Session: $Session"


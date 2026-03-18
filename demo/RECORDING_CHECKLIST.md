# 5-Minute Recording Checklist

Use this checklist to publish a quick reproducible demo.

## Before Recording

1. Set `OPENAI_API_KEY` in terminal.
2. Run startup script:
   - Windows: `deploy/scripts/start-autoloop.ps1`
   - Linux: `deploy/scripts/start-autoloop.sh`
3. Verify:
   - `http://127.0.0.1:8787/health`
   - `http://127.0.0.1:5174`

## Recording Flow (Target: 5 minutes)

1. Show project root and `README.md` badges.
2. Run demo script:
   - Windows: `demo/e2e-5min.ps1`
   - Linux: `demo/e2e-5min.sh`
3. Show generated dashboard/replay outputs for `demo-5min`.
4. Open dashboard UI and highlight:
   - Capability governance panel
   - Replay timeline
   - Route forensics
5. End with `cargo test --workspace` summary.

## Upload Notes

- Title suggestion: `AutoLoop v0.1.0-alpha 5-minute E2E walkthrough`
- Include:
  - commit hash
  - OS info
  - command list used during recording


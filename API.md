# AutoLoop API (Current)

This is the currently exposed dashboard/control-plane API surface.

## Health

- `GET /health`
  - Returns service status for the dashboard backend.

## Dashboard Data

- `GET /api/dashboard/:session`
  - Returns dashboard snapshot for a session.
- `GET /api/replay/:session`
  - Returns replay timeline for a session.
- `GET /api/catalog/:session`
  - Returns capability catalog view for a session.

## Events

- `GET /api/events`
  - SSE stream for dashboard events.
  - Example events:
    - `capability_governed`
    - `operator_settings_saved`

## Governance

- `POST /api/capabilities/govern`
  - Body:
    - `action` (`verify` | `deprecate` | `rollback`)
    - `tool` (capability name)

## Operator Settings

- `GET /api/operator/settings`
  - Returns language/vendor/model/API base/API key settings used by the UI/runtime scripts.
- `POST /api/operator/settings`
  - Persists operator settings into runtime artifacts (`deploy/runtime/operator-settings.json`).

## Notes

- This API is intentionally minimal and powers the local operations dashboard.
- Runtime artifacts are local operational outputs and should not be committed.


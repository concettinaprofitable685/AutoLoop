# AutoLoop Dashboard UI

This dashboard is a Vue 3 + TypeScript + Vite frontend for AutoLoop operations.

It is designed to visualize:

- verifier readiness
- route and treatment share
- capability governance health
- research backend health
- proxy pressure and forensics
- graph memory and global graph state
- forged capability catalog status

## Local development

```powershell
cd D:\AutoLoop\autoloop-app\dashboard-ui
npm install
npm run dev
```

## Production build

```powershell
cd D:\AutoLoop\autoloop-app\dashboard-ui
npm install
npm run build
```

## Current validation note

The workspace files are in place, but this environment could not run the Vite toolchain because the local package-manager setup is incomplete:

- `node` was available
- `npm` and `npx` were missing their CLI modules
- `corepack pnpm` could not fetch packages in the restricted environment

So the UI source is present and ready, but frontend dependency installation and build verification still need to be run in a normal Node environment.

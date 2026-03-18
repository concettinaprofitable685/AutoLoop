# Monitoring

This folder contains starter monitoring assets for AutoLoop.

- `prometheus-rules.yaml`
  - verifier, route quality, backend health, proxy pressure, and capability health alerts
- `alertmanager-config.yaml`
  - webhook-based routing with a dedicated critical lane

Expected metrics to expose from the runtime or sidecar:

- `autoloop_verifier_score`
- `autoloop_proxy_open_circuit_total`
- `autoloop_capability_health_score`
- `autoloop_research_health_status`
- `autoloop_proxy_available_total`
- `autoloop_route_correctness_score`

Recommended deployment pairings:

- `deploy/k8s/autoloop-servicemonitor.yaml`
  - Prometheus Operator scrape target for the runtime
- `deploy/k8s/autoloop-external-secret.yaml`
  - External Secrets Operator template for pulling runtime tokens from a secret manager
- `deploy/k8s/autoloop-secret-template.yaml`
  - runtime secret contract for research, model, proxy, and alert tokens
- `deploy/k8s/browserless-secret-template.yaml`
  - Browserless token template
- `deploy/monitoring/prometheus.yml`
  - local Prometheus scrape config for the bundled compose stack

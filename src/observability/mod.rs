use anyhow::Result;
use autoloop_spacetimedb_adapter::SpacetimeDb;
use serde::{Deserialize, Serialize};
use tracing::{info, warn};

use crate::{
    config::{DeploymentConfig, ObservabilityConfig},
    orchestration::{ExecutionReport, SwarmOutcome},
    tools::CapabilityLifecycleReport,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraceEnvelope {
    pub session_id: String,
    pub span_name: String,
    pub level: String,
    pub detail: String,
    pub created_at_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouteAnalyticsRecord {
    pub session_id: String,
    pub total_reports: usize,
    pub treatment_share: f32,
    pub guarded_reports: usize,
    pub top_tools: Vec<String>,
    pub top_servers: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FailureForensicsRecord {
    pub session_id: String,
    pub failing_tasks: Vec<String>,
    pub blocked_tools: Vec<String>,
    pub approval_gated_tools: Vec<String>,
    pub primary_failure_mode: String,
    pub summary: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardSnapshot {
    pub session_id: String,
    pub route_analytics: RouteAnalyticsRecord,
    pub failure_forensics: FailureForensicsRecord,
    pub validation_ready: bool,
    pub verifier_score: f32,
    pub capability_failures: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationsReport {
    pub session_id: String,
    pub session_summary: String,
    pub task_summary: Vec<String>,
    pub capability_summary: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductOpsSnapshot {
    pub session_id: String,
    pub dashboard_endpoint_hint: String,
    pub capability_lifecycle: CapabilityLifecycleReport,
    pub verifier_queue_depth: usize,
}

#[derive(Debug, Clone)]
pub struct ObservabilityKernel {
    config: ObservabilityConfig,
    deployment: DeploymentConfig,
}

impl ObservabilityKernel {
    pub fn from_config(config: &ObservabilityConfig, deployment: &DeploymentConfig) -> Self {
        Self {
            config: config.clone(),
            deployment: deployment.clone(),
        }
    }

    pub fn validate(&self) -> Result<()> {
        if self.config.enabled && self.config.report_top_k == 0 {
            anyhow::bail!("observability.report_top_k must be greater than 0");
        }
        Ok(())
    }

    pub async fn persist_swarm_observability(
        &self,
        db: &SpacetimeDb,
        session_id: &str,
        outcome: &SwarmOutcome,
        capability_lifecycle: &CapabilityLifecycleReport,
        verifier_queue_depth: usize,
    ) -> Result<()> {
        if !self.config.enabled {
            return Ok(());
        }

        let route_analytics = self.route_analytics(session_id, &outcome.execution_reports);
        let failure_forensics = self.failure_forensics(session_id, &outcome.execution_reports);
        let dashboard = DashboardSnapshot {
            session_id: session_id.to_string(),
            route_analytics: route_analytics.clone(),
            failure_forensics: failure_forensics.clone(),
            validation_ready: outcome.validation.ready,
            verifier_score: outcome.verifier_report.overall_score,
            capability_failures: outcome
                .verifier_report
                .capability_regression
                .failing_tools
                .clone(),
        };
        let operations_report = self.operations_report(session_id, outcome);
        let product_ops = ProductOpsSnapshot {
            session_id: session_id.to_string(),
            dashboard_endpoint_hint: format!("spacetimedb://observability/{session_id}/dashboard"),
            capability_lifecycle: capability_lifecycle.clone(),
            verifier_queue_depth,
        };

        db.upsert_json_knowledge(
            format!("observability:{session_id}:route-analytics"),
            &route_analytics,
            "observability",
        )
        .await?;
        db.upsert_json_knowledge(
            format!("observability:{session_id}:failure-forensics"),
            &failure_forensics,
            "observability",
        )
        .await?;
        db.upsert_json_knowledge(
            format!("observability:{session_id}:dashboard"),
            &dashboard,
            "observability",
        )
        .await?;
        db.upsert_json_knowledge(
            format!("observability:{session_id}:operations-report"),
            &operations_report,
            "observability",
        )
        .await?;
        db.upsert_json_knowledge(
            format!("observability:{session_id}:product-ops"),
            &product_ops,
            "observability",
        )
        .await?;

        let trace = TraceEnvelope {
            session_id: session_id.to_string(),
            span_name: "swarm.completed".into(),
            level: if outcome.validation.ready { "info".into() } else { "warn".into() },
            detail: format!(
                "validation_ready={} verifier_score={:.2} deployment_profile={}",
                outcome.validation.ready, outcome.verifier_report.overall_score, self.deployment.profile
            ),
            created_at_ms: crate::orchestration::current_time_ms(),
        };
        db.upsert_json_knowledge(
            format!("observability:{session_id}:trace:{}", trace.created_at_ms),
            &trace,
            "observability",
        )
        .await?;

        info!(
            session_id = session_id,
            verifier_score = outcome.verifier_report.overall_score,
            validation_ready = outcome.validation.ready,
            "persisted swarm observability snapshot"
        );
        if !failure_forensics.failing_tasks.is_empty() {
            warn!(
                session_id = session_id,
                failure_mode = failure_forensics.primary_failure_mode,
                "failure forensics detected"
            );
        }

        Ok(())
    }

    fn route_analytics(&self, session_id: &str, reports: &[ExecutionReport]) -> RouteAnalyticsRecord {
        let total_reports = reports.len();
        let treatment_count = reports
            .iter()
            .filter(|report| report.route_variant == "treatment")
            .count();
        let guarded_reports = reports
            .iter()
            .filter(|report| !report.guard_decision.eq_ignore_ascii_case("allow"))
            .count();
        let mut top_tools = reports
            .iter()
            .filter_map(|report| report.tool_used.clone())
            .collect::<Vec<_>>();
        top_tools.sort();
        top_tools.dedup();
        top_tools.truncate(self.config.report_top_k);
        let mut top_servers = reports
            .iter()
            .filter_map(|report| report.mcp_server.clone())
            .collect::<Vec<_>>();
        top_servers.sort();
        top_servers.dedup();
        top_servers.truncate(self.config.report_top_k);

        RouteAnalyticsRecord {
            session_id: session_id.to_string(),
            total_reports,
            treatment_share: if total_reports == 0 {
                0.0
            } else {
                treatment_count as f32 / total_reports as f32
            },
            guarded_reports,
            top_tools,
            top_servers,
        }
    }

    fn failure_forensics(&self, session_id: &str, reports: &[ExecutionReport]) -> FailureForensicsRecord {
        let failing_tasks = reports
            .iter()
            .filter(|report| report.outcome_score <= 0)
            .map(|report| format!("{}:{}", report.task.role, report.task.objective))
            .collect::<Vec<_>>();
        let blocked_tools = reports
            .iter()
            .filter(|report| report.guard_decision.eq_ignore_ascii_case("blocked"))
            .filter_map(|report| report.tool_used.clone())
            .collect::<Vec<_>>();
        let approval_gated_tools = reports
            .iter()
            .filter(|report| report.guard_decision.eq_ignore_ascii_case("requiresapproval"))
            .filter_map(|report| report.tool_used.clone())
            .collect::<Vec<_>>();
        let primary_failure_mode = if !blocked_tools.is_empty() {
            "runtime-guard-blocked"
        } else if !approval_gated_tools.is_empty() {
            "approval-gated"
        } else if !failing_tasks.is_empty() {
            "execution-regression"
        } else {
            "none"
        };

        FailureForensicsRecord {
            session_id: session_id.to_string(),
            summary: format!(
                "failure_mode={} failing_tasks={} blocked_tools={} approval_gated={}",
                primary_failure_mode,
                failing_tasks.len(),
                blocked_tools.len(),
                approval_gated_tools.len()
            ),
            failing_tasks,
            blocked_tools,
            approval_gated_tools,
            primary_failure_mode: primary_failure_mode.into(),
        }
    }

    fn operations_report(&self, session_id: &str, outcome: &SwarmOutcome) -> OperationsReport {
        OperationsReport {
            session_id: session_id.to_string(),
            session_summary: format!(
                "validation_ready={} verifier={:?} tasks={} profile={}",
                outcome.validation.ready,
                outcome.verifier_report.verdict,
                outcome.tasks.len(),
                self.deployment.profile
            ),
            task_summary: outcome
                .tasks
                .iter()
                .map(|task| format!("{} -> {}", task.role, task.objective))
                .collect(),
            capability_summary: outcome
                .verifier_report
                .capability_regression
                .cases
                .iter()
                .map(|case| {
                    format!(
                        "{} v{} status={} approval={} health={:.2}",
                        case.tool_name, case.version, case.status, case.approval_status, case.health_score
                    )
                })
                .take(self.config.report_top_k)
                .collect(),
        }
    }
}

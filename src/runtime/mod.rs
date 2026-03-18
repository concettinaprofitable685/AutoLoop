use std::collections::HashMap;
use std::path::{Path, PathBuf};

use anyhow::{Result, bail};
use autoloop_spacetimedb_adapter::{PermissionAction, ScheduleEvent, SpacetimeDb};
use serde::{Deserialize, Serialize};
use tokio::{process::Command, time::{Duration, timeout}};

use crate::{
    config::RuntimeConfig,
    hooks::LearningTask,
    orchestration::{ExecutionReport, RequirementBrief, RoutingContext},
    tools::{CapabilityRisk, CapabilityStatus, ExecutionStep, ExecutionStepResult, ForgedMcpToolManifest, ToolRegistry, RenderedCommandSpec, build_command_spec},
};

#[derive(Debug, Clone)]
pub struct ResourceLimits {
    pub max_parallel_agents: usize,
    pub max_memory_mb: u32,
}

#[derive(Debug, Clone)]
pub struct McpExecutionProfile {
    pub enabled: bool,
    pub allow_network_tools: bool,
    pub tool_breaker_failure_threshold: u32,
    pub tool_breaker_cooldown_ms: u64,
    pub mcp_breaker_failure_threshold: u32,
    pub mcp_breaker_cooldown_ms: u64,
}

#[derive(Debug, Clone)]
pub struct RuntimeKernel {
    pub limits: ResourceLimits,
    pub mcp: McpExecutionProfile,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SandboxPolicy {
    pub filesystem_allow: Vec<String>,
    pub filesystem_deny: Vec<String>,
    pub cpu_budget_ms: u64,
    pub memory_budget_mb: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum GuardDecision {
    Allow,
    RequiresApproval,
    Blocked,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeGuardReport {
    pub decision: GuardDecision,
    pub attempts_allowed: u8,
    pub timeout_secs: u64,
    pub reason: String,
    pub breaker_key: String,
    pub sandbox_policy: Option<SandboxPolicy>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SandboxedExecutionResult {
    pub executable: String,
    pub args: Vec<String>,
    pub working_directory: String,
    pub exit_code: Option<i32>,
    pub stdout: String,
    pub stderr: String,
    pub timed_out: bool,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CircuitState {
    pub scope_key: String,
    pub failure_count: u32,
    pub success_count: u32,
    pub phase: CircuitPhase,
    pub opened_at_ms: Option<u64>,
    pub last_failure_ms: Option<u64>,
    pub last_success_ms: Option<u64>,
    pub cooldown_ms: u64,
    pub threshold: u32,
    pub last_reason: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum CircuitPhase {
    #[default]
    Closed,
    Open,
    HalfOpen,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpDispatchRequest {
    pub session_id: String,
    pub tool_name: String,
    pub payload: String,
    pub actor_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvaluationProtocol {
    pub protocol_name: String,
    pub metric_name: String,
    pub time_budget_secs: u64,
    pub mutable_by_agent: bool,
    pub acceptance_checks: Vec<String>,
    pub required_verifiers: Vec<String>,
    pub immutable_artifacts: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvaluationResult {
    pub metric_name: String,
    pub score: f32,
    pub summary: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IterationRecord {
    pub actions: Vec<ExecutionStepResult>,
    pub evaluation: EvaluationResult,
    pub keep: bool,
    pub rollback_reason: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum VerifierVerdict {
    Pass,
    NeedsIteration,
    Reject,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskLevelJudgement {
    pub task_role: String,
    pub satisfied: bool,
    pub score: f32,
    pub summary: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouteCorrectnessReport {
    pub task_role: String,
    pub tool_name: Option<String>,
    pub route_variant: String,
    pub aligned_with_catalog: bool,
    pub aligned_with_graph: bool,
    pub guard_ok: bool,
    pub score: f32,
    pub summary: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilityRegressionCase {
    pub tool_name: String,
    pub capability_id: String,
    pub version: u32,
    pub status: String,
    pub approval_status: String,
    pub health_score: f32,
    pub passed: bool,
    pub summary: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilityRegressionSuite {
    pub suite_name: String,
    pub all_passed: bool,
    pub score: f32,
    pub failing_tools: Vec<String>,
    pub cases: Vec<CapabilityRegressionCase>,
    pub summary: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerifierReport {
    pub verifier_name: String,
    pub verdict: VerifierVerdict,
    pub overall_score: f32,
    pub summary: String,
    pub task_judgements: Vec<TaskLevelJudgement>,
    pub route_reports: Vec<RouteCorrectnessReport>,
    pub capability_regression: CapabilityRegressionSuite,
    pub recommended_actions: Vec<String>,
}

impl RuntimeKernel {
    pub fn from_config(config: &RuntimeConfig) -> Self {
        Self {
            limits: ResourceLimits {
                max_parallel_agents: config.max_parallel_agents,
                max_memory_mb: config.max_memory_mb,
            },
            mcp: McpExecutionProfile {
                enabled: config.mcp_enabled,
                allow_network_tools: config.allow_network_tools,
                tool_breaker_failure_threshold: config.tool_breaker_failure_threshold,
                tool_breaker_cooldown_ms: config.tool_breaker_cooldown_ms,
                mcp_breaker_failure_threshold: config.mcp_breaker_failure_threshold,
                mcp_breaker_cooldown_ms: config.mcp_breaker_cooldown_ms,
            },
        }
    }

    pub fn validate(&self) -> Result<()> {
        if self.limits.max_parallel_agents == 0 {
            bail!("runtime.max_parallel_agents must be greater than 0");
        }
        if self.limits.max_memory_mb == 0 {
            bail!("runtime.max_memory_mb must be greater than 0");
        }
        if self.mcp.tool_breaker_failure_threshold == 0 {
            bail!("runtime.tool_breaker_failure_threshold must be greater than 0");
        }
        if self.mcp.mcp_breaker_failure_threshold == 0 {
            bail!("runtime.mcp_breaker_failure_threshold must be greater than 0");
        }
        Ok(())
    }

    pub async fn dispatch_mcp_event(
        &self,
        db: &SpacetimeDb,
        request: McpDispatchRequest,
    ) -> Result<ScheduleEvent> {
        db.enforce_permission(&request.actor_id, PermissionAction::Dispatch)
            .await?;

        db.create_schedule_event(
            request.session_id,
            "mcp.dispatch".into(),
            request.tool_name,
            request.payload,
            request.actor_id,
        )
        .await
    }

    pub async fn guard_tool_execution_with_state(
        &self,
        db: &SpacetimeDb,
        actor_id: &str,
        tool_name: &str,
        manifest: Option<&ForgedMcpToolManifest>,
    ) -> Result<RuntimeGuardReport> {
        let mut report = self.guard_tool_execution(actor_id, tool_name, manifest);
        if report.decision != GuardDecision::Allow {
            return Ok(report);
        }

        let now_ms = current_time_ms();
        let tool_key = self.tool_circuit_key(tool_name);
        if let Some(tool_state) = self.load_circuit_state(db, &tool_key).await? {
            if let Some(block_reason) = self.circuit_block_reason(&tool_state, now_ms) {
                report.decision = GuardDecision::Blocked;
                report.attempts_allowed = 0;
                report.reason = format!("tool circuit open: {block_reason}");
                report.breaker_key = tool_key;
                return Ok(report);
            }
            if tool_state.phase == CircuitPhase::HalfOpen {
                report.attempts_allowed = 1;
                report.reason = format!("tool circuit is half-open: {}", report.reason);
                report.breaker_key = tool_key;
            }
        }

        if let Some(server_name) = server_name_for(tool_name, manifest) {
            let server_key = self.server_circuit_key(&server_name);
            if let Some(server_state) = self.load_circuit_state(db, &server_key).await? {
                if let Some(block_reason) = self.circuit_block_reason(&server_state, now_ms) {
                    report.decision = GuardDecision::Blocked;
                    report.attempts_allowed = 0;
                    report.reason = format!("mcp circuit open: {block_reason}");
                    report.breaker_key = server_key;
                    return Ok(report);
                }
                if server_state.phase == CircuitPhase::HalfOpen {
                    report.attempts_allowed = report.attempts_allowed.min(1);
                    report.reason = format!("mcp circuit is half-open: {}", report.reason);
                    report.breaker_key = server_key;
                }
            }
        }

        Ok(report)
    }

    pub fn guard_tool_execution(
        &self,
        actor_id: &str,
        tool_name: &str,
        manifest: Option<&ForgedMcpToolManifest>,
    ) -> RuntimeGuardReport {
        let timeout_secs = manifest
            .and_then(|manifest| manifest.success_signal.as_ref().map(|_| 120))
            .unwrap_or(90);
        let breaker_key = format!("{actor_id}:{tool_name}");

        if let Some(manifest) = manifest {
            if manifest.status != CapabilityStatus::Active {
                return RuntimeGuardReport {
                    decision: GuardDecision::Blocked,
                    attempts_allowed: 0,
                    timeout_secs,
                    reason: format!("capability status {:?} is not runnable", manifest.status),
                    breaker_key,
                    sandbox_policy: Some(self.sandbox_policy_for(tool_name, manifest)),
                };
            }
            if manifest.approval_status != crate::tools::ApprovalStatus::Verified {
                return RuntimeGuardReport {
                    decision: GuardDecision::RequiresApproval,
                    attempts_allowed: 0,
                    timeout_secs,
                    reason: "capability is not verified yet".into(),
                    breaker_key,
                    sandbox_policy: Some(self.sandbox_policy_for(tool_name, manifest)),
                };
            }
            if manifest.health_score < 0.4 {
                return RuntimeGuardReport {
                    decision: GuardDecision::Blocked,
                    attempts_allowed: 0,
                    timeout_secs,
                    reason: format!("capability health {:.2} is below runtime minimum", manifest.health_score),
                    breaker_key,
                    sandbox_policy: Some(self.sandbox_policy_for(tool_name, manifest)),
                };
            }
            if manifest.requires_gate() || (self.mcp.allow_network_tools && manifest.risk == CapabilityRisk::High) {
                return RuntimeGuardReport {
                    decision: GuardDecision::RequiresApproval,
                    attempts_allowed: 1,
                    timeout_secs,
                    reason: "capability risk requires approval gate".into(),
                    breaker_key,
                    sandbox_policy: Some(self.sandbox_policy_for(tool_name, manifest)),
                };
            }
        }

        RuntimeGuardReport {
            decision: GuardDecision::Allow,
            attempts_allowed: 2,
            timeout_secs,
            reason: "runtime guard allows bounded execution".into(),
            breaker_key,
            sandbox_policy: manifest.map(|manifest| self.sandbox_policy_for(tool_name, manifest)),
        }
    }

    pub async fn record_execution_outcome(
        &self,
        db: &SpacetimeDb,
        report: &ExecutionReport,
    ) -> Result<Vec<CircuitState>> {
        let Some(tool_name) = report.tool_used.as_deref() else {
            return Ok(Vec::new());
        };
        if !report.guard_decision.eq_ignore_ascii_case("allow") {
            return Ok(Vec::new());
        }

        let now_ms = current_time_ms();
        let succeeded = report.outcome_score > 0
            && !report.output.to_ascii_lowercase().contains("failed")
            && !report.output.to_ascii_lowercase().contains("blocked");

        let mut updates = Vec::new();

        let tool_key = self.tool_circuit_key(tool_name);
        let tool_state = self
            .load_circuit_state(db, &tool_key)
            .await?
            .unwrap_or_else(|| self.default_circuit_state(tool_key.clone(), false));
        let updated_tool_state =
            self.transition_circuit_state(tool_state, succeeded, now_ms, report.output.clone());
        self.persist_circuit_state(db, &updated_tool_state).await?;
        updates.push(updated_tool_state);

        if let Some(server_name) = report
            .mcp_server
            .clone()
            .or_else(|| server_name_for(tool_name, None))
        {
            let server_key = self.server_circuit_key(&server_name);
            let server_state = self
                .load_circuit_state(db, &server_key)
                .await?
                .unwrap_or_else(|| self.default_circuit_state(server_key.clone(), true));
            let updated_server_state = self.transition_circuit_state(
                server_state,
                succeeded,
                now_ms,
                report.output.clone(),
            );
            self.persist_circuit_state(db, &updated_server_state).await?;
            updates.push(updated_server_state);
        }

        Ok(updates)
    }

    pub async fn execute_sandboxed_manifest(
        &self,
        manifest: &ForgedMcpToolManifest,
        arguments: &str,
        policy: &SandboxPolicy,
    ) -> Result<SandboxedExecutionResult> {
        let spec = build_command_spec(manifest, arguments)?;
        validate_command_spec(&spec, policy)?;

        let working_directory = resolve_working_directory(spec.working_directory.as_deref())?;
        enforce_working_directory_policy(&working_directory, policy)?;

        let mut command = Command::new(&spec.executable);
        command
            .args(&spec.args)
            .current_dir(&working_directory)
            .stdin(std::process::Stdio::null())
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .kill_on_drop(true);

        let child = command.spawn()?;
        let timeout_budget = Duration::from_secs(policy.cpu_budget_ms.max(1000) / 1000);
        match timeout(timeout_budget, child.wait_with_output()).await {
            Ok(output) => {
                let output = output?;
                Ok(SandboxedExecutionResult {
                    executable: spec.executable,
                    args: spec.args,
                    working_directory: working_directory.to_string_lossy().to_string(),
                    exit_code: output.status.code(),
                    stdout: String::from_utf8_lossy(&output.stdout).trim().to_string(),
                    stderr: String::from_utf8_lossy(&output.stderr).trim().to_string(),
                    timed_out: false,
                })
            }
            Err(_) => Ok(SandboxedExecutionResult {
                executable: spec.executable,
                args: spec.args,
                working_directory: working_directory.to_string_lossy().to_string(),
                exit_code: None,
                stdout: String::new(),
                stderr: "sandbox timeout exceeded".into(),
                timed_out: true,
            }),
        }
    }

    pub async fn circuit_snapshot(
        &self,
        db: &SpacetimeDb,
    ) -> Result<HashMap<String, CircuitState>> {
        let mut snapshot = HashMap::new();
        for record in db.list_knowledge_by_prefix("metrics:circuit:").await? {
            if let Ok(state) = serde_json::from_str::<CircuitState>(&record.value) {
                snapshot.insert(record.key, state);
            }
        }
        Ok(snapshot)
    }

    fn default_circuit_state(&self, scope_key: String, is_mcp: bool) -> CircuitState {
        CircuitState {
            scope_key,
            failure_count: 0,
            success_count: 0,
            phase: CircuitPhase::Closed,
            opened_at_ms: None,
            last_failure_ms: None,
            last_success_ms: None,
            cooldown_ms: if is_mcp {
                self.mcp.mcp_breaker_cooldown_ms
            } else {
                self.mcp.tool_breaker_cooldown_ms
            },
            threshold: if is_mcp {
                self.mcp.mcp_breaker_failure_threshold
            } else {
                self.mcp.tool_breaker_failure_threshold
            },
            last_reason: None,
        }
    }

    fn transition_circuit_state(
        &self,
        mut state: CircuitState,
        succeeded: bool,
        now_ms: u64,
        reason: String,
    ) -> CircuitState {
        if succeeded {
            state.success_count = state.success_count.saturating_add(1);
            state.failure_count = 0;
            state.phase = CircuitPhase::Closed;
            state.last_success_ms = Some(now_ms);
            state.opened_at_ms = None;
            state.last_reason = Some("execution succeeded and circuit closed".into());
            return state;
        }

        state.failure_count = state.failure_count.saturating_add(1);
        state.last_failure_ms = Some(now_ms);
        state.last_reason = Some(reason);
        if state.failure_count >= state.threshold {
            state.phase = CircuitPhase::Open;
            state.opened_at_ms = Some(now_ms);
        } else if state.phase == CircuitPhase::HalfOpen {
            state.phase = CircuitPhase::Open;
            state.opened_at_ms = Some(now_ms);
        }
        state
    }

    fn circuit_block_reason(&self, state: &CircuitState, now_ms: u64) -> Option<String> {
        match state.phase {
            CircuitPhase::Closed => None,
            CircuitPhase::HalfOpen => None,
            CircuitPhase::Open => {
                let opened_at = state.opened_at_ms.unwrap_or(now_ms);
                if now_ms.saturating_sub(opened_at) >= state.cooldown_ms {
                    None
                } else {
                    Some(format!(
                        "cooldown active for {} ms (failures: {})",
                        state.cooldown_ms.saturating_sub(now_ms.saturating_sub(opened_at)),
                        state.failure_count
                    ))
                }
            }
        }
    }

    async fn load_circuit_state(
        &self,
        db: &SpacetimeDb,
        key: &str,
    ) -> Result<Option<CircuitState>> {
        let state = db
            .get_knowledge(key)
            .await?
            .and_then(|record| serde_json::from_str::<CircuitState>(&record.value).ok())
            .map(|mut state| {
                if state.phase == CircuitPhase::Open {
                    if let Some(opened_at) = state.opened_at_ms {
                        if current_time_ms().saturating_sub(opened_at) >= state.cooldown_ms {
                            state.phase = CircuitPhase::HalfOpen;
                        }
                    }
                }
                state
            });
        Ok(state)
    }

    async fn persist_circuit_state(
        &self,
        db: &SpacetimeDb,
        state: &CircuitState,
    ) -> Result<()> {
        db.upsert_knowledge(
            state.scope_key.clone(),
            serde_json::to_string(state)?,
            "runtime-circuit".into(),
        )
        .await?;
        Ok(())
    }

    fn tool_circuit_key(&self, tool_name: &str) -> String {
        format!("metrics:circuit:tool:{tool_name}")
    }

    fn server_circuit_key(&self, server_name: &str) -> String {
        format!("metrics:circuit:server:{server_name}")
    }

    fn sandbox_policy_for(
        &self,
        tool_name: &str,
        manifest: &ForgedMcpToolManifest,
    ) -> SandboxPolicy {
        let mut filesystem_allow = vec![".".into(), "./workspace".into()];
        if manifest.scope == crate::tools::CapabilityScope::Session {
            filesystem_allow.push("./workspace/session".into());
        }
        if let Some(working_directory) = &manifest.working_directory {
            filesystem_allow.push(working_directory.clone());
        }
        let mut filesystem_deny = vec!["./.git".into(), "./deploy/secrets".into()];
        if manifest.risk == CapabilityRisk::High || tool_name.contains("deploy") {
            filesystem_deny.push("/".into());
        }
        SandboxPolicy {
            filesystem_allow,
            filesystem_deny,
            cpu_budget_ms: if manifest.risk == CapabilityRisk::High { 15_000 } else { 6_000 },
            memory_budget_mb: if manifest.risk == CapabilityRisk::High {
                self.limits.max_memory_mb.min(768)
            } else {
                self.limits.max_memory_mb.min(384)
            },
        }
    }

    pub fn evaluation_protocol(&self) -> EvaluationProtocol {
        EvaluationProtocol {
            protocol_name: "immutable-objective-protocol".into(),
            metric_name: "objective_score".into(),
            time_budget_secs: 300,
            mutable_by_agent: false,
            acceptance_checks: vec![
                "acceptance-criteria-coverage".into(),
                "task-level-judge".into(),
                "route-correctness".into(),
                "capability-regression".into(),
            ],
            required_verifiers: vec![
                "verifier-agent".into(),
                "task-judge".into(),
                "route-auditor".into(),
                "capability-regression-suite".into(),
            ],
            immutable_artifacts: vec![
                "acceptance_criteria".into(),
                "routing_catalog".into(),
                "guard_decision".into(),
            ],
        }
    }

    pub fn evaluate_candidate(
        &self,
        baseline_score: f32,
        candidate_score: f32,
    ) -> EvaluationResult {
        EvaluationResult {
            metric_name: self.evaluation_protocol().metric_name,
            score: candidate_score,
            summary: format!(
                "Immutable protocol compared candidate {:.6} against baseline {:.6}. Lower is better.",
                candidate_score, baseline_score
            ),
        }
    }

    pub async fn run_iteration_loop(
        &self,
        tools: &ToolRegistry,
        actions: &[ExecutionStep],
        baseline_score: f32,
        candidate_score: f32,
    ) -> Result<IterationRecord> {
        let action_results = tools.execute_plan(actions).await?;
        let evaluation = self.evaluate_candidate(baseline_score, candidate_score);
        let keep = evaluation.score < baseline_score;
        let rollback_reason = (!keep).then(|| {
            format!(
                "candidate {:.6} did not improve over baseline {:.6}",
                evaluation.score, baseline_score
            )
        });

        Ok(IterationRecord {
            actions: action_results,
            evaluation,
            keep,
            rollback_reason,
        })
    }

    pub fn learn_from_iteration_failure(&self, record: &IterationRecord) -> Option<LearningTask> {
        if record.keep {
            return None;
        }

        Some(LearningTask {
            hook_name: "optimization-reflexion".into(),
            anchor: "iteration-regression".into(),
            reason: record
                .rollback_reason
                .clone()
                .unwrap_or_else(|| "iteration failed immutable objective".into()),
            priority: "high".into(),
        })
    }

    pub fn verify_swarm_outcome(
        &self,
        brief: &RequirementBrief,
        routing: &RoutingContext,
        reports: &[ExecutionReport],
        tools: &ToolRegistry,
    ) -> VerifierReport {
        let task_judgements = self.judge_tasks(brief, reports);
        let route_reports = self.judge_routes(routing, reports, tools);
        let capability_regression = self.run_capability_regression_suite(tools);

        let task_score = average_score(task_judgements.iter().map(|item| item.score));
        let route_score = average_score(route_reports.iter().map(|item| item.score));
        let acceptance_coverage = acceptance_coverage(brief, reports);
        let overall_score = ((task_score + route_score + acceptance_coverage + capability_regression.score)
            / 4.0)
            .clamp(0.0, 1.0);

        let mut recommended_actions = Vec::new();
        if acceptance_coverage < 0.8 {
            recommended_actions.push("verifier: expand task evidence for frozen acceptance criteria".into());
        }
        if route_score < 0.65 {
            recommended_actions.push("verifier: audit route selection against catalog and graph signals".into());
        }
        if !capability_regression.all_passed {
            recommended_actions.push("verifier: deprecate or roll back failing capabilities".into());
        }
        if routing.pending_event_count > 0 {
            recommended_actions.push("verifier: drain pending scheduled events before completion".into());
        }

        let verdict = if !capability_regression.all_passed {
            VerifierVerdict::Reject
        } else if routing.pending_event_count > 0
            || acceptance_coverage < 0.75
            || task_score < 0.6
            || route_score < 0.6
        {
            VerifierVerdict::NeedsIteration
        } else {
            VerifierVerdict::Pass
        };

        VerifierReport {
            verifier_name: "verifier-agent".into(),
            verdict: verdict.clone(),
            overall_score,
            summary: format!(
                "Verifier {:?}: acceptance {:.2}, task {:.2}, route {:.2}, capability {:.2}.",
                verdict, acceptance_coverage, task_score, route_score, capability_regression.score
            ),
            task_judgements,
            route_reports,
            capability_regression,
            recommended_actions,
        }
    }

    fn judge_tasks(
        &self,
        brief: &RequirementBrief,
        reports: &[ExecutionReport],
    ) -> Vec<TaskLevelJudgement> {
        reports
            .iter()
            .map(|report| {
                let output = report.output.to_ascii_lowercase();
                let positive_signal = report.outcome_score > 0
                    && !output.contains("blocked")
                    && !output.contains("requires approval");
                let criterion_hits = brief
                    .acceptance_criteria
                    .iter()
                    .filter(|criterion| output.contains(&criterion.to_ascii_lowercase()))
                    .count();
                let criteria_score = if brief.acceptance_criteria.is_empty() {
                    1.0
                } else {
                    criterion_hits as f32 / brief.acceptance_criteria.len() as f32
                };
                let score = ((if positive_signal { 0.6 } else { 0.1 }) + criteria_score * 0.4)
                    .clamp(0.0, 1.0);
                TaskLevelJudgement {
                    task_role: report.task.role.clone(),
                    satisfied: positive_signal && score >= 0.55,
                    score,
                    summary: format!(
                        "{} satisfied={} criterion_hits={}/{} outcome_score={}",
                        report.task.agent_name,
                        positive_signal && score >= 0.55,
                        criterion_hits,
                        brief.acceptance_criteria.len(),
                        report.outcome_score
                    ),
                }
            })
            .collect()
    }

    fn judge_routes(
        &self,
        routing: &RoutingContext,
        reports: &[ExecutionReport],
        tools: &ToolRegistry,
    ) -> Vec<RouteCorrectnessReport> {
        reports
            .iter()
            .map(|report| {
                let aligned_with_catalog = report.tool_used.as_ref().is_none_or(|tool_name| {
                    if report.task.role != "Execution" {
                        return true;
                    }
                    tools.forged_tool_names().iter().any(|name| name == tool_name)
                        || tool_name == "cli::forge_mcp_tool"
                });
                let aligned_with_graph = match report.tool_used.as_deref() {
                    Some(tool_name) if tool_name == "cli::forge_mcp_tool" => {
                        routing.forged_tool_coverage == 0 || routing.graph_signals.prefers_cli_execution
                    }
                    Some(tool_name) if tool_name.starts_with("mcp::") => {
                        routing.graph_signals.prefers_mcp_execution || routing.forged_tool_coverage > 0
                    }
                    Some(_) => routing.graph_signals.prefers_cli_execution,
                    None => true,
                };
                let guard_ok =
                    report.guard_decision.eq_ignore_ascii_case("allow")
                        || report.guard_decision.eq_ignore_ascii_case("provider");
                let mut score = 0.2f32;
                if aligned_with_catalog {
                    score += 0.35;
                }
                if aligned_with_graph {
                    score += 0.25;
                }
                if guard_ok {
                    score += 0.2;
                }
                RouteCorrectnessReport {
                    task_role: report.task.role.clone(),
                    tool_name: report.tool_used.clone(),
                    route_variant: report.route_variant.clone(),
                    aligned_with_catalog,
                    aligned_with_graph,
                    guard_ok,
                    score: score.clamp(0.0, 1.0),
                    summary: format!(
                        "catalog={} graph={} guard={} variant={}",
                        aligned_with_catalog, aligned_with_graph, guard_ok, report.route_variant
                    ),
                }
            })
            .collect()
    }

    pub fn run_capability_regression_suite(&self, tools: &ToolRegistry) -> CapabilityRegressionSuite {
        let cases = tools
            .manifests()
            .into_iter()
            .map(|manifest| {
                let passed = manifest.status == CapabilityStatus::Active
                    && manifest.approval_status == crate::tools::ApprovalStatus::Verified
                    && manifest.health_score >= 0.55
                    && !(manifest.risk == CapabilityRisk::High && manifest.health_score < 0.7);
                let summary = if passed {
                    "capability satisfies executable governance baseline".to_string()
                } else {
                    format!(
                        "status={:?} approval={:?} health={:.2} risk={:?}",
                        manifest.status, manifest.approval_status, manifest.health_score, manifest.risk
                    )
                };
                CapabilityRegressionCase {
                    tool_name: manifest.registered_tool_name,
                    capability_id: manifest.capability_id,
                    version: manifest.version,
                    status: format!("{:?}", manifest.status),
                    approval_status: format!("{:?}", manifest.approval_status),
                    health_score: manifest.health_score,
                    passed,
                    summary,
                }
            })
            .collect::<Vec<_>>();
        let failing_tools = cases
            .iter()
            .filter(|case| !case.passed)
            .map(|case| case.tool_name.clone())
            .collect::<Vec<_>>();
        let passed_count = cases.iter().filter(|case| case.passed).count();
        let score = if cases.is_empty() {
            1.0
        } else {
            passed_count as f32 / cases.len() as f32
        };

        CapabilityRegressionSuite {
            suite_name: "capability-regression-suite".into(),
            all_passed: failing_tools.is_empty(),
            score,
            failing_tools,
            summary: if cases.is_empty() {
                "No forged capabilities were registered, so the regression suite is vacuously green.".into()
            } else {
                format!(
                    "{} of {} capabilities satisfy the verifier baseline.",
                    passed_count,
                    cases.len()
                )
            },
            cases,
        }
    }
}

fn current_time_ms() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|duration| duration.as_millis() as u64)
        .unwrap_or(0)
}

fn resolve_working_directory(working_directory: Option<&str>) -> Result<PathBuf> {
    let requested = working_directory.unwrap_or(".");
    let path = Path::new(requested);
    let absolute = if path.is_absolute() {
        path.to_path_buf()
    } else {
        std::env::current_dir()?.join(path)
    };
    Ok(absolute)
}

fn validate_command_spec(
    spec: &RenderedCommandSpec,
    policy: &SandboxPolicy,
) -> Result<()> {
    let executable = spec.executable.to_ascii_lowercase();
    let blocked_executables = ["powershell", "pwsh", "cmd", "bash", "sh"];
    if blocked_executables
        .iter()
        .any(|blocked| executable.ends_with(blocked) || executable.contains(&format!("{blocked}.")))
    {
        bail!("sandbox blocked interpreter-style executable: {}", spec.executable);
    }

    let denied_prefixes = policy
        .filesystem_deny
        .iter()
        .map(|entry| entry.to_ascii_lowercase())
        .collect::<Vec<_>>();
    for argument in &spec.args {
        let lowered = argument.to_ascii_lowercase();
        if denied_prefixes.iter().any(|prefix| lowered.contains(prefix)) {
            bail!("sandbox blocked denied path-like argument: {argument}");
        }
    }
    Ok(())
}

fn enforce_working_directory_policy(path: &Path, policy: &SandboxPolicy) -> Result<()> {
    let normalized = path.to_string_lossy().replace('\\', "/").to_ascii_lowercase();
    let allowed = policy.filesystem_allow.iter().any(|entry| {
        let candidate = entry.replace('\\', "/").to_ascii_lowercase();
        let candidate = candidate.trim_start_matches("./");
        candidate.is_empty()
            || normalized.contains(candidate)
            || normalized.ends_with(candidate)
    });
    if !allowed {
        bail!("sandbox blocked working directory outside allowlist: {}", path.display());
    }
    let denied = policy.filesystem_deny.iter().any(|entry| {
        let candidate = entry.replace('\\', "/").to_ascii_lowercase();
        normalized.contains(candidate.trim_start_matches("./"))
    });
    if denied {
        bail!("sandbox blocked working directory inside denylist: {}", path.display());
    }
    Ok(())
}

fn server_name_for(
    tool_name: &str,
    manifest: Option<&ForgedMcpToolManifest>,
) -> Option<String> {
    manifest
        .map(|manifest| manifest.server.clone())
        .or_else(|| {
            let mut segments = tool_name.split("::");
            match (segments.next(), segments.next()) {
                (Some("mcp"), Some(server)) => Some(server.to_string()),
                _ => None,
            }
        })
}

fn average_score(values: impl Iterator<Item = f32>) -> f32 {
    let values = values.collect::<Vec<_>>();
    if values.is_empty() {
        1.0
    } else {
        values.iter().sum::<f32>() / values.len() as f32
    }
}

fn acceptance_coverage(brief: &RequirementBrief, reports: &[ExecutionReport]) -> f32 {
    if brief.acceptance_criteria.is_empty() {
        return 1.0;
    }
    let combined = reports
        .iter()
        .map(|report| report.output.to_ascii_lowercase())
        .collect::<Vec<_>>()
        .join("\n");
    let hits = brief
        .acceptance_criteria
        .iter()
        .filter(|criterion| combined.contains(&criterion.to_ascii_lowercase()))
        .count();
    hits as f32 / brief.acceptance_criteria.len() as f32
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use autoloop_spacetimedb_adapter::{SpacetimeBackend, SpacetimeDbConfig};

    use crate::{
        config::AppConfig,
        orchestration::{ExecutionReport, RequirementBrief, RoutingContext, SwarmTask},
        tools::{ApprovalStatus, CapabilityRisk, CapabilityScope, CapabilityStatus, CliOutputMode, ForgedMcpToolManifest},
    };
    use serde_json::json;

    fn manifest(risk: CapabilityRisk, approval_status: ApprovalStatus, status: CapabilityStatus) -> ForgedMcpToolManifest {
        ForgedMcpToolManifest {
            capability_id: "capability:test".into(),
            registered_tool_name: "mcp::local-mcp::test".into(),
            delegate_tool_name: "mcp::local-mcp::invoke".into(),
            server: "local-mcp".into(),
            capability_name: "test".into(),
            purpose: "test purpose".into(),
            executable: "test-cli".into(),
            command_template: "test-cli run".into(),
            payload_template: json!({}),
            output_mode: CliOutputMode::Json,
            working_directory: Some(".".into()),
            success_signal: Some("completed".into()),
            help_text: "help".into(),
            skill_markdown: "# skill".into(),
            examples: vec![],
            version: 1,
            lineage_key: "capability:test".into(),
            status,
            approval_status,
            health_score: 0.7,
            scope: CapabilityScope::TaskFamily,
            tags: vec![],
            risk,
            requested_by: "cli-agent".into(),
            created_at_ms: 0,
            updated_at_ms: 0,
            approved_at_ms: None,
            rollback_to_version: None,
        }
    }

    #[test]
    fn runtime_guard_requires_approval_for_high_risk_capability() {
        let runtime = RuntimeKernel::from_config(&AppConfig::default().runtime);
        let report = runtime.guard_tool_execution(
            "session-1",
            "mcp::local-mcp::test",
            Some(&manifest(
                CapabilityRisk::High,
                ApprovalStatus::Verified,
                CapabilityStatus::Active,
            )),
        );
        assert_eq!(report.decision, GuardDecision::RequiresApproval);
    }

    #[test]
    fn runtime_guard_blocks_unverified_capability() {
        let runtime = RuntimeKernel::from_config(&AppConfig::default().runtime);
        let report = runtime.guard_tool_execution(
            "session-1",
            "mcp::local-mcp::test",
            Some(&manifest(
                CapabilityRisk::Low,
                ApprovalStatus::Pending,
                CapabilityStatus::PendingVerification,
            )),
        );
        assert_eq!(report.decision, GuardDecision::Blocked);
    }

    #[test]
    fn verifier_rejects_capability_regression_failures() {
        let config = AppConfig::default();
        let runtime = RuntimeKernel::from_config(&config.runtime);
        let tools = ToolRegistry::from_config(&config.tools);
        tools.hydrate_manifest(manifest(
            CapabilityRisk::High,
            ApprovalStatus::Pending,
            CapabilityStatus::PendingVerification,
        ));
        let brief = RequirementBrief {
            anchor_id: "anchor:session-1".into(),
            original_request: "execute via catalog".into(),
            clarified_goal: "execute via catalog".into(),
            frozen_scope: "capability-catalog execution".into(),
            open_questions: vec![],
            acceptance_criteria: vec!["mcp".into()],
            clarification_turns: vec![],
            confirmation_required: false,
        };
        let routing = RoutingContext {
            history_records: vec![],
            execution_metrics: vec![],
            graph_signals: Default::default(),
            pending_event_count: 0,
            agent_reputations: HashMap::new(),
            learning_evidence: vec![],
            skill_success_rate: 0.0,
            causal_confidence: 0.0,
            forged_tool_coverage: 0,
            session_ab_stats: None,
            task_ab_stats: Default::default(),
            tool_ab_stats: Default::default(),
            server_ab_stats: Default::default(),
            route_biases: vec![],
        };
        let reports = vec![ExecutionReport {
            task: SwarmTask {
                task_id: "execution-catalog-requires-approval".into(),
                agent_name: "execution-agent".into(),
                role: "Execution".into(),
                objective: "execute via catalog".into(),
                depends_on: Vec::new(),
            },
            output: "execution requires approval".into(),
            tool_used: Some("mcp::local-mcp::test".into()),
            mcp_server: Some("local-mcp".into()),
            invocation_payload: Some("{}".into()),
            outcome_score: -6,
            route_variant: "control".into(),
            control_score: 1,
            treatment_score: 1,
            guard_decision: "RequiresApproval".into(),
        }];

        let verifier = runtime.verify_swarm_outcome(&brief, &routing, &reports, &tools);

        assert_eq!(verifier.verdict, VerifierVerdict::Reject);
        assert!(!verifier.capability_regression.all_passed);
        assert!(!verifier.capability_regression.failing_tools.is_empty());
    }

    #[test]
    fn verifier_passes_healthy_verified_catalog_execution() {
        let config = AppConfig::default();
        let runtime = RuntimeKernel::from_config(&config.runtime);
        let tools = ToolRegistry::from_config(&config.tools);
        tools.hydrate_manifest(manifest(
            CapabilityRisk::Low,
            ApprovalStatus::Verified,
            CapabilityStatus::Active,
        ));
        let brief = RequirementBrief {
            anchor_id: "anchor:session-1".into(),
            original_request: "execute via catalog".into(),
            clarified_goal: "execute via catalog".into(),
            frozen_scope: "capability-catalog execution".into(),
            open_questions: vec![],
            acceptance_criteria: vec!["mcp".into(), "completed".into()],
            clarification_turns: vec![],
            confirmation_required: false,
        };
        let mut graph_signals = crate::rag::GraphRoutingSignals::default();
        graph_signals.prefers_mcp_execution = true;
        let routing = RoutingContext {
            history_records: vec![],
            execution_metrics: vec![],
            graph_signals,
            pending_event_count: 0,
            agent_reputations: HashMap::new(),
            learning_evidence: vec![],
            skill_success_rate: 0.0,
            causal_confidence: 0.0,
            forged_tool_coverage: 1,
            session_ab_stats: None,
            task_ab_stats: Default::default(),
            tool_ab_stats: Default::default(),
            server_ab_stats: Default::default(),
            route_biases: vec![],
        };
        let reports = vec![ExecutionReport {
            task: SwarmTask {
                task_id: "execution-catalog-healthy".into(),
                agent_name: "execution-agent".into(),
                role: "Execution".into(),
                objective: "execute via catalog".into(),
                depends_on: Vec::new(),
            },
            output: "mcp execution completed successfully".into(),
            tool_used: Some("mcp::local-mcp::test".into()),
            mcp_server: Some("local-mcp".into()),
            invocation_payload: Some("{}".into()),
            outcome_score: 4,
            route_variant: "control".into(),
            control_score: 4,
            treatment_score: 4,
            guard_decision: "Allow".into(),
        }];

        let verifier = runtime.verify_swarm_outcome(&brief, &routing, &reports, &tools);

        assert_eq!(verifier.verdict, VerifierVerdict::Pass);
        assert!(verifier.capability_regression.all_passed);
        assert!(verifier.overall_score > 0.7);
    }

    #[tokio::test]
    async fn circuit_state_opens_after_threshold_failures() {
        let mut config = AppConfig::default();
        config.runtime.tool_breaker_failure_threshold = 2;
        config.runtime.tool_breaker_cooldown_ms = 60_000;
        let runtime = RuntimeKernel::from_config(&config.runtime);
        let db = SpacetimeDb::from_config(&SpacetimeDbConfig {
            enabled: true,
            backend: SpacetimeBackend::InMemory,
            uri: "http://spacetimedb:3000".into(),
            module_name: "autoloop_core".into(),
            namespace: "autoloop".into(),
            pool_size: 4,
        });
        let executable_manifest = manifest(
            CapabilityRisk::Low,
            ApprovalStatus::Verified,
            CapabilityStatus::Active,
        );
        let failure = ExecutionReport {
            task: SwarmTask {
                task_id: "breaker-failure".into(),
                agent_name: "execution-agent".into(),
                role: "Execution".into(),
                objective: "execute failing tool".into(),
                depends_on: Vec::new(),
            },
            output: "failed with an error".into(),
            tool_used: Some("mcp::local-mcp::test".into()),
            mcp_server: Some("local-mcp".into()),
            invocation_payload: Some("{}".into()),
            outcome_score: -5,
            route_variant: "control".into(),
            control_score: -5,
            treatment_score: -5,
            guard_decision: "Allow".into(),
        };

        runtime
            .record_execution_outcome(&db, &failure)
            .await
            .expect("first failure");
        runtime
            .record_execution_outcome(&db, &failure)
            .await
            .expect("second failure");

        let guard = runtime
            .guard_tool_execution_with_state(
                &db,
                "session-1",
                "mcp::local-mcp::test",
                Some(&executable_manifest),
            )
            .await
            .expect("guard");

        assert_eq!(guard.decision, GuardDecision::Blocked);
        assert!(guard.reason.contains("circuit open"));
    }

    #[tokio::test]
    async fn circuit_state_recovers_into_half_open_after_cooldown() {
        let mut config = AppConfig::default();
        config.runtime.tool_breaker_failure_threshold = 1;
        config.runtime.tool_breaker_cooldown_ms = 1;
        let runtime = RuntimeKernel::from_config(&config.runtime);
        let db = SpacetimeDb::from_config(&SpacetimeDbConfig {
            enabled: true,
            backend: SpacetimeBackend::InMemory,
            uri: "http://spacetimedb:3000".into(),
            module_name: "autoloop_core".into(),
            namespace: "autoloop".into(),
            pool_size: 4,
        });
        let executable_manifest = manifest(
            CapabilityRisk::Low,
            ApprovalStatus::Verified,
            CapabilityStatus::Active,
        );
        let failure = ExecutionReport {
            task: SwarmTask {
                task_id: "breaker-half-open".into(),
                agent_name: "execution-agent".into(),
                role: "Execution".into(),
                objective: "execute failing tool".into(),
                depends_on: Vec::new(),
            },
            output: "failed with an error".into(),
            tool_used: Some("mcp::local-mcp::test".into()),
            mcp_server: Some("local-mcp".into()),
            invocation_payload: Some("{}".into()),
            outcome_score: -5,
            route_variant: "control".into(),
            control_score: -5,
            treatment_score: -5,
            guard_decision: "Allow".into(),
        };

        runtime
            .record_execution_outcome(&db, &failure)
            .await
            .expect("failure");
        std::thread::sleep(std::time::Duration::from_millis(2));

        let guard = runtime
            .guard_tool_execution_with_state(
                &db,
                "session-2",
                "mcp::local-mcp::test",
                Some(&executable_manifest),
            )
            .await
            .expect("guard");

        assert_eq!(guard.decision, GuardDecision::Allow);
        assert_eq!(guard.attempts_allowed, 1);
        assert!(guard.reason.contains("half-open"));
    }

    #[tokio::test]
    async fn sandbox_executor_runs_real_command_for_verified_manifest() {
        let runtime = RuntimeKernel::from_config(&AppConfig::default().runtime);
        let mut executable_manifest = manifest(
            CapabilityRisk::Low,
            ApprovalStatus::Verified,
            CapabilityStatus::Active,
        );
        executable_manifest.executable = "rustc".into();
        executable_manifest.command_template = "rustc --version".into();
        executable_manifest.working_directory = Some(".".into());
        let policy = runtime.sandbox_policy_for("mcp::local-mcp::test", &executable_manifest);

        let result = runtime
            .execute_sandboxed_manifest(&executable_manifest, "{}", &policy)
            .await
            .expect("sandbox execution");

        assert!(!result.timed_out);
        assert_eq!(result.exit_code, Some(0));
        assert!(result.stdout.to_ascii_lowercase().contains("rustc"));
    }

    #[tokio::test]
    async fn sandbox_executor_blocks_interpreter_style_commands() {
        let runtime = RuntimeKernel::from_config(&AppConfig::default().runtime);
        let mut executable_manifest = manifest(
            CapabilityRisk::Low,
            ApprovalStatus::Verified,
            CapabilityStatus::Active,
        );
        executable_manifest.executable = "powershell".into();
        executable_manifest.command_template = "powershell -Command Get-Date".into();
        executable_manifest.working_directory = Some(".".into());
        let policy = runtime.sandbox_policy_for("mcp::local-mcp::test", &executable_manifest);

        let error = runtime
            .execute_sandboxed_manifest(&executable_manifest, "{}", &policy)
            .await
            .expect_err("interpreter should be blocked");

        assert!(error.to_string().contains("interpreter-style executable"));
    }
}

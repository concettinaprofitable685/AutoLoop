use anyhow::{Result, bail};
use autoloop_spacetimedb_adapter::SpacetimeDb;
use serde::Serialize;

use crate::{
    config::HooksConfig,
    runtime::IterationRecord,
    tools::ExecutionStep,
};

#[derive(Debug, Clone)]
pub struct HookSpec {
    pub name: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct LearningTask {
    pub hook_name: String,
    pub anchor: String,
    pub reason: String,
    pub priority: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct IterationLearningHook {
    pub proposal_anchor: String,
    pub actions: Vec<ExecutionStep>,
    pub reason: String,
}

#[derive(Debug, Clone)]
pub struct HookRegistry {
    hooks: Vec<HookSpec>,
    pub learning_hooks_enabled: bool,
}

impl HookRegistry {
    pub fn from_config(config: &HooksConfig) -> Self {
        Self {
            hooks: config
                .builtin
                .iter()
                .cloned()
                .map(|name| HookSpec { name })
                .collect(),
            learning_hooks_enabled: config.learning_hooks_enabled,
        }
    }

    pub fn validate(&self) -> Result<()> {
        if self.learning_hooks_enabled && self.hooks.is_empty() {
            bail!("learning hooks are enabled but no hooks are registered");
        }
        Ok(())
    }

    pub fn len(&self) -> usize {
        self.hooks.len()
    }

    pub fn augment_system_prompt(&self, prompt: &str) -> String {
        if self.learning_hooks_enabled {
            format!(
                "{prompt}\n\n[Hooks] Self-learning hooks enabled. Track anchors, identify knowledge gaps, and schedule follow-up learning when evidence is weak."
            )
        } else {
            prompt.to_string()
        }
    }

    pub fn plan_learning_tasks(
        &self,
        user_input: &str,
        assistant_response: &str,
    ) -> Vec<LearningTask> {
        if !self.learning_hooks_enabled {
            return Vec::new();
        }

        let anchors = extract_anchors(user_input);
        let mut tasks = Vec::new();

        for anchor in anchors {
            if !assistant_response.to_ascii_lowercase().contains(&anchor.to_ascii_lowercase()) {
                tasks.push(LearningTask {
                    hook_name: "self-learn".into(),
                    anchor,
                    reason: "anchor mentioned by user but not covered in the response".into(),
                    priority: "high".into(),
                });
            }
        }

        if response_signals_uncertainty(assistant_response) {
            tasks.push(LearningTask {
                hook_name: "self-learn".into(),
                anchor: "knowledge-gap".into(),
                reason: "assistant response signaled uncertainty and should trigger knowledge-gap review".into(),
                priority: "medium".into(),
            });
        }

        tasks
    }

    pub async fn schedule_learning_tasks(
        &self,
        db: &SpacetimeDb,
        session_id: &str,
        actor_id: &str,
        user_input: &str,
        assistant_response: &str,
    ) -> Result<Vec<LearningTask>> {
        let tasks = self.plan_learning_tasks(user_input, assistant_response);
        if tasks.is_empty() || !db.has_permission(actor_id, autoloop_spacetimedb_adapter::PermissionAction::Dispatch).await? {
            return Ok(tasks);
        }

        for task in &tasks {
            db.create_schedule_event(
                session_id.to_string(),
                "hooks.self_learn".into(),
                "mcp::local-mcp::invoke".into(),
                serde_json::to_string(task)?,
                actor_id.to_string(),
            )
            .await?;
        }

        Ok(tasks)
    }

    pub fn plan_iteration_hooks(
        &self,
        anchor: &str,
        record: &IterationRecord,
    ) -> Vec<IterationLearningHook> {
        if !self.learning_hooks_enabled {
            return Vec::new();
        }

        let mut hooks = Vec::new();
        if !record.keep {
            hooks.push(IterationLearningHook {
                proposal_anchor: anchor.to_string(),
                actions: record.actions.iter().map(|result| result.action.clone()).collect(),
                reason: record
                    .rollback_reason
                    .clone()
                    .unwrap_or_else(|| "iteration did not improve immutable objective".into()),
            });
        }

        hooks
    }
}

fn extract_anchors(user_input: &str) -> Vec<String> {
    user_input
        .split_whitespace()
        .filter_map(|token| {
            let normalized = token
                .trim_matches(|ch: char| !ch.is_ascii_alphanumeric() && ch != ':' && ch != '#' && ch != '-')
                .to_ascii_lowercase();

            if normalized.starts_with("anchor:") {
                return Some(normalized.trim_start_matches("anchor:").to_string());
            }
            if normalized.starts_with('#') && normalized.len() > 1 {
                return Some(normalized.trim_start_matches('#').to_string());
            }

            None
        })
        .collect()
}

fn response_signals_uncertainty(response: &str) -> bool {
    let lowered = response.to_ascii_lowercase();
    [
        "not sure",
        "unclear",
        "need more data",
        "insufficient context",
        "i don't know",
    ]
    .iter()
    .any(|pattern| lowered.contains(pattern))
}

#[cfg(test)]
mod tests {
    use super::*;
    use autoloop_spacetimedb_adapter::{PermissionAction, SpacetimeBackend, SpacetimeDbConfig};

    #[test]
    fn hooks_detect_anchor_and_gap_signals() {
        let hooks = HookRegistry {
            hooks: vec![HookSpec {
                name: "self-learn".into(),
            }],
            learning_hooks_enabled: true,
        };

        let tasks = hooks.plan_learning_tasks(
            "Please expand anchor:GraphRAG and #spacetimedb",
            "I am not sure about the retrieval path yet.",
        );

        assert!(tasks.iter().any(|task| task.anchor == "graphrag"));
        assert!(tasks.iter().any(|task| task.anchor == "spacetimedb"));
        assert!(tasks.iter().any(|task| task.anchor == "knowledge-gap"));
    }

    #[tokio::test]
    async fn hooks_schedule_tasks_into_spacetimedb() {
        let hooks = HookRegistry {
            hooks: vec![HookSpec {
                name: "self-learn".into(),
            }],
            learning_hooks_enabled: true,
        };
        let db = SpacetimeDb::from_config(&SpacetimeDbConfig {
            enabled: true,
            backend: SpacetimeBackend::InMemory,
            uri: "http://spacetimedb:3000".into(),
            module_name: "autoloop_core".into(),
            namespace: "autoloop".into(),
            pool_size: 4,
        });

        db.grant_permissions("agent-1", vec![PermissionAction::Dispatch])
            .await
            .expect("grant");

        let tasks = hooks
            .schedule_learning_tasks(
                &db,
                "session-1",
                "agent-1",
                "Investigate anchor:spacetimedb",
                "Need more data before I can answer completely.",
            )
            .await
            .expect("schedule");

        let events = db.list_schedule_events("session-1").await.expect("events");

        assert!(!tasks.is_empty());
        assert_eq!(events.len(), tasks.len());
        assert!(events.iter().all(|event| event.topic == "hooks.self_learn"));
    }
}

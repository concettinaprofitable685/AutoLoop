use anyhow::{Result, bail};
use autoloop_spacetimedb_adapter::SpacetimeDb;

use crate::{
    config::AgentConfig,
    hooks::HookRegistry,
    memory::MemorySubsystem,
    providers::{ChatMessage, ProviderRegistry},
    security::SecurityPolicy,
    session::SessionStore,
    tools::ToolRegistry,
};

#[derive(Clone)]
pub struct AgentRuntime {
    config: AgentConfig,
    providers: ProviderRegistry,
    tools: ToolRegistry,
    sessions: SessionStore,
    memory: MemorySubsystem,
    hooks: HookRegistry,
    security: SecurityPolicy,
    spacetimedb: SpacetimeDb,
}

impl AgentRuntime {
    pub fn new(
        config: AgentConfig,
        providers: ProviderRegistry,
        tools: ToolRegistry,
        sessions: SessionStore,
        memory: MemorySubsystem,
        hooks: HookRegistry,
        security: SecurityPolicy,
        spacetimedb: SpacetimeDb,
    ) -> Self {
        Self {
            config,
            providers,
            tools,
            sessions,
            memory,
            hooks,
            security,
            spacetimedb,
        }
    }

    pub fn validate(&self) -> Result<()> {
        if self.config.max_iterations == 0 {
            bail!("agent.max_iterations must be greater than 0");
        }
        if self.config.memory_window == 0 {
            bail!("agent.memory_window must be greater than 0");
        }
        Ok(())
    }

    pub async fn process_message(&self, session_id: &str, content: &str) -> Result<String> {
        let security_report = self.security.inspect_text(content);
        if security_report.blocked {
            let refusal = format!(
                "Request blocked by security policy: {}",
                security_report
                    .findings
                    .into_iter()
                    .map(|finding| finding.detail)
                    .collect::<Vec<_>>()
                    .join("; ")
            );
            self.sessions
                .append_assistant_message(session_id, &refusal)
                .await;
            return Ok(refusal);
        }

        self.sessions.append_user_message(session_id, content).await;
        let history = self.sessions.history(session_id).await;

        let mut messages = Vec::new();
        let memory_context = self
            .memory
            .build_memory_context_with_learning(&self.spacetimedb, session_id, content, &history)
            .await
            .unwrap_or_else(|_| self.memory.build_memory_context_for(content, &history));
        let evolution_summary = self
            .spacetimedb
            .get_knowledge(&format!("memory:{session_id}:self-evolution"))
            .await
            .ok()
            .flatten()
            .map(|record| record.value);
        let research_summary = self
            .spacetimedb
            .get_knowledge(&format!("research:{session_id}:report"))
            .await
            .ok()
            .flatten()
            .map(|record| record.value);
        let capability_hints = self
            .spacetimedb
            .list_knowledge_by_prefix(&format!("memory:{session_id}:evolution-proposal:"))
            .await
            .unwrap_or_default()
            .into_iter()
            .filter_map(|record| serde_json::from_str::<serde_json::Value>(&record.value).ok())
            .filter_map(|value| {
                value
                    .get("tool_name")
                    .and_then(|tool_name| tool_name.as_str())
                    .map(str::to_string)
            })
            .collect::<Vec<_>>();
        let prompt_overlay = self.providers.derive_prompt_policy(
            content,
            evolution_summary.as_deref(),
            research_summary.as_deref(),
            &capability_hints,
        );
        let adaptive_guidance = if prompt_overlay.directives.is_empty() {
            String::new()
        } else {
            format!(
                "\n\n# Adaptive Guidance\n{}\n\n# Policy Rationale\n{}",
                prompt_overlay
                    .directives
                    .iter()
                    .map(|line| format!("- {line}"))
                    .collect::<Vec<_>>()
                    .join("\n"),
                prompt_overlay.rationale
            )
        };
        let system_prompt = self.hooks.augment_system_prompt(&format!(
            "{}\n\n# Memory Targets\n{}",
            self.config.system_prompt, memory_context
        ));
        let system_prompt = format!("{system_prompt}{adaptive_guidance}");

        messages.push(ChatMessage {
            role: "system".into(),
            content: system_prompt,
        });
        messages.extend(history);

        let mut iteration = 0usize;
        loop {
            iteration += 1;
            if iteration > self.config.max_iterations {
                let stopped = "Agent stopped after reaching the max iteration limit.".to_string();
                self.sessions
                    .append_assistant_message(session_id, &stopped)
                    .await;
                return Ok(stopped);
            }

            let response = self
                .providers
                .chat_with_policy(&messages, prompt_overlay.preferred_model.as_deref())
                .await?;
            if response.tool_calls.is_empty() {
                let final_text = response
                    .content
                    .unwrap_or_else(|| "No response content.".to_string());
                self.sessions
                    .append_assistant_message(session_id, &final_text)
                    .await;
                let _ = self
                    .spacetimedb
                    .upsert_agent_state(
                        session_id.to_string(),
                        content.to_string(),
                        Some(final_text.clone()),
                    )
                    .await;
                let _ = self
                    .hooks
                    .schedule_learning_tasks(
                        &self.spacetimedb,
                        session_id,
                        session_id,
                        content,
                        &final_text,
                    )
                    .await;
                return Ok(final_text);
            }

            if let Some(content) = response.content {
                messages.push(ChatMessage {
                    role: "assistant".into(),
                    content,
                });
            }

            for call in response.tool_calls {
                let tool_report = self
                    .security
                    .inspect_tool_call(&self.spacetimedb, session_id, &call.name, &call.arguments)
                    .await?;
                if tool_report.blocked {
                    let blocked_message = format!(
                        "Tool call '{}' blocked by security policy: {}",
                        call.name,
                        tool_report
                            .findings
                            .into_iter()
                            .map(|finding| finding.detail)
                            .collect::<Vec<_>>()
                            .join("; ")
                    );
                    self.sessions
                        .append_assistant_message(session_id, &blocked_message)
                        .await;
                    return Ok(blocked_message);
                }

                let result = self.tools.execute(&call.name, &call.arguments).await?;
                self.sessions
                    .append_tool_message(session_id, &result.name, &result.content)
                    .await;
                messages.push(ChatMessage {
                    role: "tool".into(),
                    content: result.content,
                });
            }
        }
    }
}

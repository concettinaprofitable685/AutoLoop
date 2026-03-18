use anyhow::{Result, anyhow, bail};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

use super::{Tool, ToolRegistry, ToolResult};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RenderedCommandSpec {
    pub executable: String,
    pub args: Vec<String>,
    pub display_command: String,
    pub working_directory: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum CliOutputMode {
    Human,
    Json,
    Both,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum CapabilityStatus {
    PendingVerification,
    Active,
    Deprecated,
    RolledBack,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ApprovalStatus {
    Pending,
    Verified,
    Rejected,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum CapabilityScope {
    Session,
    TaskFamily,
    Global,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum CapabilityRisk {
    Low,
    Medium,
    High,
}

impl Default for CliOutputMode {
    fn default() -> Self {
        Self::Json
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForgeArgumentSpec {
    pub name: String,
    pub description: String,
    #[serde(default)]
    pub required: bool,
    #[serde(default)]
    pub example: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpToolForgeRequest {
    #[serde(default)]
    pub server: Option<String>,
    pub capability_name: String,
    pub purpose: String,
    pub executable: String,
    #[serde(default)]
    pub subcommands: Vec<String>,
    #[serde(default)]
    pub json_flag: Option<String>,
    #[serde(default)]
    pub arguments: Vec<ForgeArgumentSpec>,
    #[serde(default)]
    pub output_mode: CliOutputMode,
    #[serde(default)]
    pub success_signal: Option<String>,
    #[serde(default)]
    pub working_directory: Option<String>,
    #[serde(default)]
    pub examples: Vec<String>,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub scope: Option<CapabilityScope>,
    #[serde(default)]
    pub requested_by: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForgedMcpToolManifest {
    #[serde(default)]
    pub capability_id: String,
    pub registered_tool_name: String,
    pub delegate_tool_name: String,
    pub server: String,
    pub capability_name: String,
    pub purpose: String,
    pub executable: String,
    pub command_template: String,
    pub payload_template: Value,
    pub output_mode: CliOutputMode,
    pub working_directory: Option<String>,
    pub success_signal: Option<String>,
    pub help_text: String,
    pub skill_markdown: String,
    pub examples: Vec<String>,
    #[serde(default = "default_capability_version")]
    pub version: u32,
    #[serde(default)]
    pub lineage_key: String,
    #[serde(default = "default_capability_status")]
    pub status: CapabilityStatus,
    #[serde(default = "default_approval_status")]
    pub approval_status: ApprovalStatus,
    #[serde(default = "default_health_score")]
    pub health_score: f32,
    #[serde(default = "default_scope")]
    pub scope: CapabilityScope,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default = "default_risk")]
    pub risk: CapabilityRisk,
    #[serde(default = "default_requested_by")]
    pub requested_by: String,
    #[serde(default)]
    pub created_at_ms: u64,
    #[serde(default)]
    pub updated_at_ms: u64,
    #[serde(default)]
    pub approved_at_ms: Option<u64>,
    #[serde(default)]
    pub rollback_to_version: Option<u32>,
}

impl ForgedMcpToolManifest {
    pub fn is_executable(&self) -> bool {
        self.status == CapabilityStatus::Active
            && self.approval_status == ApprovalStatus::Verified
            && self.health_score >= 0.55
    }

    pub fn requires_gate(&self) -> bool {
        matches!(self.risk, CapabilityRisk::High | CapabilityRisk::Medium)
    }
}

impl Default for ForgedMcpToolManifest {
    fn default() -> Self {
        Self {
            capability_id: "capability:default".into(),
            registered_tool_name: "mcp::local-mcp::default".into(),
            delegate_tool_name: "mcp::local-mcp::invoke".into(),
            server: "local-mcp".into(),
            capability_name: "default".into(),
            purpose: "default capability".into(),
            executable: "autoloop-cli".into(),
            command_template: "autoloop-cli".into(),
            payload_template: json!({}),
            output_mode: CliOutputMode::Json,
            working_directory: Some(".".into()),
            success_signal: None,
            help_text: String::new(),
            skill_markdown: String::new(),
            examples: Vec::new(),
            version: 1,
            lineage_key: "capability:default".into(),
            status: CapabilityStatus::Active,
            approval_status: ApprovalStatus::Verified,
            health_score: 0.8,
            scope: CapabilityScope::TaskFamily,
            tags: Vec::new(),
            risk: CapabilityRisk::Low,
            requested_by: "cli-agent".into(),
            created_at_ms: 0,
            updated_at_ms: 0,
            approved_at_ms: None,
            rollback_to_version: None,
        }
    }
}

pub struct CliAnythingForgeTool {
    registry: ToolRegistry,
    default_server: String,
}

impl CliAnythingForgeTool {
    pub fn new(registry: ToolRegistry, default_server: impl Into<String>) -> Self {
        Self {
            registry,
            default_server: default_server.into(),
        }
    }
}

#[async_trait]
impl Tool for CliAnythingForgeTool {
    fn name(&self) -> &str {
        "cli::forge_mcp_tool"
    }

    async fn execute(&self, arguments: &str) -> Result<ToolResult> {
        let request: McpToolForgeRequest = serde_json::from_str(arguments)
            .map_err(|error| anyhow!("forge request must be valid JSON: {error}"))?;
        let server = request
            .server
            .clone()
            .unwrap_or_else(|| self.default_server.clone());
        let capability_slug = sanitize_segment(&request.capability_name);
        if capability_slug.is_empty() {
            bail!("capability_name must contain at least one alphanumeric character");
        }
        let tool_name = format!("mcp::{server}::{capability_slug}");
        let manifest = build_manifest(tool_name.clone(), server.clone(), request)?;
        self.registry.upsert_governed_manifest(manifest.clone()).await?;

        Ok(ToolResult {
            name: self.name().to_string(),
            content: serde_json::to_string_pretty(&manifest)?,
        })
    }
}

pub struct ForgedToolCatalog {
    registry: ToolRegistry,
}

impl ForgedToolCatalog {
    pub fn new(registry: ToolRegistry) -> Self {
        Self { registry }
    }
}

#[async_trait]
impl Tool for ForgedToolCatalog {
    fn name(&self) -> &str {
        "cli::list_forged_tools"
    }

    async fn execute(&self, arguments: &str) -> Result<ToolResult> {
        let requested_server = arguments.trim();
        let manifests = self
            .registry
            .manifests()
            .into_iter()
            .filter(|manifest| requested_server.is_empty() || manifest.server == requested_server)
            .collect::<Vec<_>>();

        Ok(ToolResult {
            name: self.name().to_string(),
            content: serde_json::to_string_pretty(&manifests)?,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct CapabilityMutationRequest {
    tool_name: String,
    #[serde(default)]
    health_score: Option<f32>,
}

pub struct CapabilityVerifierTool {
    registry: ToolRegistry,
}

impl CapabilityVerifierTool {
    pub fn new(registry: ToolRegistry) -> Self {
        Self { registry }
    }
}

#[async_trait]
impl Tool for CapabilityVerifierTool {
    fn name(&self) -> &str {
        "cli::verify_capability"
    }

    async fn execute(&self, arguments: &str) -> Result<ToolResult> {
        let request: CapabilityMutationRequest = serde_json::from_str(arguments)?;
        let manifest = self
            .registry
            .verify_capability(&request.tool_name)
            .await?
            .ok_or_else(|| anyhow!("unknown capability: {}", request.tool_name))?;
        Ok(ToolResult {
            name: self.name().into(),
            content: serde_json::to_string_pretty(&manifest)?,
        })
    }
}

pub struct CapabilityDeprecationTool {
    registry: ToolRegistry,
}

impl CapabilityDeprecationTool {
    pub fn new(registry: ToolRegistry) -> Self {
        Self { registry }
    }
}

#[async_trait]
impl Tool for CapabilityDeprecationTool {
    fn name(&self) -> &str {
        "cli::deprecate_capability"
    }

    async fn execute(&self, arguments: &str) -> Result<ToolResult> {
        let request: CapabilityMutationRequest = serde_json::from_str(arguments)?;
        let manifest = self
            .registry
            .deprecate_capability(&request.tool_name, request.health_score.unwrap_or(0.25))
            .await?
            .ok_or_else(|| anyhow!("unknown capability: {}", request.tool_name))?;
        Ok(ToolResult {
            name: self.name().into(),
            content: serde_json::to_string_pretty(&manifest)?,
        })
    }
}

pub struct CapabilityRollbackTool {
    registry: ToolRegistry,
}

impl CapabilityRollbackTool {
    pub fn new(registry: ToolRegistry) -> Self {
        Self { registry }
    }
}

#[async_trait]
impl Tool for CapabilityRollbackTool {
    fn name(&self) -> &str {
        "cli::rollback_capability"
    }

    async fn execute(&self, arguments: &str) -> Result<ToolResult> {
        let request: CapabilityMutationRequest = serde_json::from_str(arguments)?;
        let manifest = self
            .registry
            .rollback_capability(&request.tool_name)
            .await?
            .ok_or_else(|| anyhow!("no rollback target for capability: {}", request.tool_name))?;
        Ok(ToolResult {
            name: self.name().into(),
            content: serde_json::to_string_pretty(&manifest)?,
        })
    }
}

#[derive(Debug)]
pub struct ForgedMcpTool {
    name: String,
    manifest: ForgedMcpToolManifest,
}

impl ForgedMcpTool {
    pub fn new(name: String, manifest: ForgedMcpToolManifest) -> Self {
        Self { name, manifest }
    }
}

#[async_trait]
impl Tool for ForgedMcpTool {
    fn name(&self) -> &str {
        &self.name
    }

    async fn execute(&self, arguments: &str) -> Result<ToolResult> {
        let payload = parse_invocation(arguments)?;
        let rendered_command = render_command(&self.manifest, &payload)?;
        let response = json!({
            "delegate_tool": self.manifest.delegate_tool_name,
            "server": self.manifest.server,
            "capability_name": self.manifest.capability_name,
            "command": rendered_command,
            "arguments": payload,
            "output_mode": self.manifest.output_mode,
            "working_directory": self.manifest.working_directory,
            "success_signal": self.manifest.success_signal,
            "help_text": self.manifest.help_text,
        });

        Ok(ToolResult {
            name: self.name.clone(),
            content: serde_json::to_string_pretty(&response)?,
        })
    }
}

fn build_manifest(
    tool_name: String,
    server: String,
    request: McpToolForgeRequest,
) -> Result<ForgedMcpToolManifest> {
    if request.executable.trim().is_empty() {
        bail!("executable must not be empty");
    }

    let delegate_tool_name = format!("mcp::{server}::invoke");
    let command_template = build_command_template(&request);
    let payload_template = json!({
        "server": server,
        "capability_name": request.capability_name,
        "command": command_template,
        "arguments": request.arguments.iter().map(|argument| {
            json!({
                "name": argument.name,
                "required": argument.required,
                "example": argument.example,
            })
        }).collect::<Vec<_>>(),
        "output_mode": request.output_mode,
        "working_directory": request.working_directory,
        "success_signal": request.success_signal,
    });
    let help_text = build_help_text(&tool_name, &request);
    let skill_markdown = build_skill_markdown(&tool_name, &server, &request);
    let risk = infer_risk(&request);
    let now_ms = current_time_ms();
    let capability_id = format!("{server}:{}", sanitize_segment(&request.capability_name));
    let status = if matches!(risk, CapabilityRisk::Low) {
        CapabilityStatus::Active
    } else {
        CapabilityStatus::PendingVerification
    };
    let approval_status = if matches!(risk, CapabilityRisk::Low) {
        ApprovalStatus::Verified
    } else {
        ApprovalStatus::Pending
    };
    let approved_at_ms = matches!(approval_status, ApprovalStatus::Verified).then_some(now_ms);

    Ok(ForgedMcpToolManifest {
        capability_id: capability_id.clone(),
        registered_tool_name: tool_name,
        delegate_tool_name,
        server,
        capability_name: request.capability_name,
        purpose: request.purpose,
        executable: request.executable,
        command_template,
        payload_template,
        output_mode: request.output_mode,
        working_directory: request.working_directory,
        success_signal: request.success_signal,
        help_text,
        skill_markdown,
        examples: request.examples,
        version: 1,
        lineage_key: capability_id,
        status,
        approval_status,
        health_score: initial_health_score(&risk),
        scope: request.scope.unwrap_or(CapabilityScope::TaskFamily),
        tags: request.tags,
        risk,
        requested_by: request.requested_by.unwrap_or_else(|| "cli-agent".into()),
        created_at_ms: now_ms,
        updated_at_ms: now_ms,
        approved_at_ms,
        rollback_to_version: None,
    })
}

fn build_command_template(request: &McpToolForgeRequest) -> String {
    let mut parts = vec![request.executable.clone()];
    parts.extend(request.subcommands.clone());
    if let Some(json_flag) = &request.json_flag {
        parts.push(json_flag.clone());
    }
    for argument in &request.arguments {
        let placeholder = format!("{{{{{}}}}}", sanitize_segment(&argument.name));
        parts.push(format!("--{} {placeholder}", sanitize_segment(&argument.name)));
    }
    parts.join(" ")
}

fn build_help_text(tool_name: &str, request: &McpToolForgeRequest) -> String {
    let arg_list = request
        .arguments
        .iter()
        .map(|argument| {
            format!(
                "- {}{}: {}",
                argument.name,
                if argument.required { " (required)" } else { "" },
                argument.description
            )
        })
        .collect::<Vec<_>>()
        .join("\n");

    format!(
        "{tool_name}\nPurpose: {}\nExecutable: {}\nOutput mode: {:?}\nArguments:\n{}",
        request.purpose,
        request.executable,
        request.output_mode,
        if arg_list.is_empty() { "- none".into() } else { arg_list }
    )
}

fn build_skill_markdown(tool_name: &str, server: &str, request: &McpToolForgeRequest) -> String {
    let examples = if request.examples.is_empty() {
        "- No examples provided yet.".to_string()
    } else {
        request
            .examples
            .iter()
            .map(|example| format!("- {example}"))
            .collect::<Vec<_>>()
            .join("\n")
    };

    format!(
        "# {tool_name}\n\nUse this forged MCP tool through `{server}` to satisfy: {}\n\n## Contract\n- Deterministic JSON-first CLI wrapper\n- Self-describing argument schema\n- Suitable for CLI agents that need reusable command surfaces\n\n## Examples\n{}",
        request.purpose, examples
    )
}

fn parse_invocation(arguments: &str) -> Result<Value> {
    if arguments.trim().is_empty() {
        return Ok(json!({}));
    }

    let parsed = serde_json::from_str::<Value>(arguments)
        .map_err(|error| anyhow!("forged tool invocation must be valid JSON: {error}"))?;
    if !parsed.is_object() {
        bail!("forged tool invocation must be a JSON object");
    }
    Ok(parsed)
}

pub fn build_command_spec(
    manifest: &ForgedMcpToolManifest,
    arguments: &str,
) -> Result<RenderedCommandSpec> {
    let payload = parse_invocation(arguments)?;
    let rendered = render_command(manifest, &payload)?;
    let object = payload
        .as_object()
        .ok_or_else(|| anyhow!("forged tool invocation must be a JSON object"))?;

    let arg_specs = manifest
        .payload_template
        .get("arguments")
        .and_then(Value::as_array)
        .cloned()
        .unwrap_or_default();

    let mut args = manifest.subcommand_segments();
    for spec in arg_specs {
        let name = spec
            .get("name")
            .and_then(Value::as_str)
            .ok_or_else(|| anyhow!("argument schema missing name"))?;
        let required = spec
            .get("required")
            .and_then(Value::as_bool)
            .unwrap_or(false);
        let key = sanitize_segment(name);
        match object.get(name).or_else(|| object.get(&key)) {
            Some(value) => {
                args.push(format!("--{key}"));
                args.push(render_value(value));
            }
            None if required => bail!("missing required forged tool argument: {name}"),
            None => {}
        }
    }
    Ok(RenderedCommandSpec {
        executable: manifest.executable.clone(),
        args,
        display_command: rendered,
        working_directory: manifest.working_directory.clone(),
    })
}

fn render_command(manifest: &ForgedMcpToolManifest, payload: &Value) -> Result<String> {
    let mut rendered = manifest.command_template.clone();
    let object = payload
        .as_object()
        .ok_or_else(|| anyhow!("forged tool invocation must be a JSON object"))?;

    let arg_specs = manifest
        .payload_template
        .get("arguments")
        .and_then(Value::as_array)
        .cloned()
        .unwrap_or_default();

    for spec in arg_specs {
        let name = spec
            .get("name")
            .and_then(Value::as_str)
            .ok_or_else(|| anyhow!("argument schema missing name"))?;
        let required = spec
            .get("required")
            .and_then(Value::as_bool)
            .unwrap_or(false);
        let key = sanitize_segment(name);
        let placeholder = format!("{{{{{key}}}}}");
        match object.get(name).or_else(|| object.get(&key)) {
            Some(value) => {
                let value = render_value(value);
                rendered = rendered.replace(&placeholder, &shell_escape(&value));
            }
            None if required => bail!("missing required forged tool argument: {name}"),
            None => {
                rendered = rendered.replace(&format!(" --{key} {placeholder}"), "");
            }
        }
    }

    Ok(rendered)
}

impl ForgedMcpToolManifest {
    fn subcommand_segments(&self) -> Vec<String> {
        let command = self.command_template.trim();
        let executable = self.executable.trim();
        let tokens = command
            .strip_prefix(executable)
            .unwrap_or(command)
            .split_whitespace()
            .filter(|segment| !segment.is_empty())
            .map(|segment| segment.to_string())
            .collect::<Vec<_>>();
        let mut segments = Vec::new();
        let mut index = 0usize;
        while index < tokens.len() {
            let token = &tokens[index];
            if token == executable {
                index += 1;
                continue;
            }
            if token.contains("{{") {
                index += 1;
                continue;
            }
            if token.starts_with("--")
                && tokens
                    .get(index + 1)
                    .is_some_and(|next| next.contains("{{"))
            {
                index += 2;
                continue;
            }
            segments.push(token.clone());
            index += 1;
        }
        segments
    }
}

fn render_value(value: &Value) -> String {
    match value {
        Value::Null => "null".into(),
        Value::Bool(value) => value.to_string(),
        Value::Number(value) => value.to_string(),
        Value::String(value) => value.clone(),
        Value::Array(values) => values.iter().map(render_value).collect::<Vec<_>>().join(","),
        Value::Object(_) => value.to_string(),
    }
}

fn shell_escape(value: &str) -> String {
    if value.chars().all(|ch| ch.is_ascii_alphanumeric() || "-_./:".contains(ch)) {
        value.to_string()
    } else {
        format!("\"{}\"", value.replace('"', "\\\""))
    }
}

pub fn sanitize_segment(value: &str) -> String {
    let mut sanitized = String::new();
    let mut previous_dash = false;

    for ch in value.chars() {
        if ch.is_ascii_alphanumeric() {
            sanitized.push(ch.to_ascii_lowercase());
            previous_dash = false;
        } else if !previous_dash {
            sanitized.push('-');
            previous_dash = true;
        }
    }

    sanitized.trim_matches('-').to_string()
}

fn infer_risk(request: &McpToolForgeRequest) -> CapabilityRisk {
    let purpose = request.purpose.to_ascii_lowercase();
    let executable = request.executable.to_ascii_lowercase();
    if purpose.contains("delete")
        || purpose.contains("deploy")
        || purpose.contains("network")
        || executable.contains("powershell")
        || executable.contains("bash")
    {
        CapabilityRisk::High
    } else if purpose.contains("write") || purpose.contains("modify") || purpose.contains("patch") {
        CapabilityRisk::Medium
    } else {
        CapabilityRisk::Low
    }
}

fn initial_health_score(risk: &CapabilityRisk) -> f32 {
    match risk {
        CapabilityRisk::Low => 0.82,
        CapabilityRisk::Medium => 0.65,
        CapabilityRisk::High => 0.45,
    }
}

fn current_time_ms() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};

    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as u64
}

fn default_capability_version() -> u32 { 1 }
fn default_capability_status() -> CapabilityStatus { CapabilityStatus::Active }
fn default_approval_status() -> ApprovalStatus { ApprovalStatus::Verified }
fn default_health_score() -> f32 { 0.8 }
fn default_scope() -> CapabilityScope { CapabilityScope::TaskFamily }
fn default_risk() -> CapabilityRisk { CapabilityRisk::Low }
fn default_requested_by() -> String { "cli-agent".into() }

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::ToolsConfig;
    use autoloop_spacetimedb_adapter::{SpacetimeBackend, SpacetimeDb, SpacetimeDbConfig};

    #[tokio::test]
    async fn forge_tool_registers_new_mcp_capability() {
        let registry = ToolRegistry::from_config(&ToolsConfig {
            builtin: vec!["read_file".into()],
            allow_shell: false,
            mcp_servers: vec!["local-mcp".into()],
        });

        let result = registry
            .execute(
                "cli::forge_mcp_tool",
                r#"{
                    "server":"local-mcp",
                    "capability_name":"image batch export",
                    "purpose":"Turn batch exports into a reusable MCP surface",
                    "executable":"image-cli",
                    "subcommands":["batch","export"],
                    "json_flag":"--json",
                    "arguments":[
                        {"name":"input","description":"input directory","required":true},
                        {"name":"format","description":"output format","required":false,"example":"png"}
                    ],
                    "output_mode":"json",
                    "success_signal":"completed"
                }"#,
            )
            .await
            .expect("forge tool");

        assert!(result.content.contains("\"registered_tool_name\": \"mcp::local-mcp::image-batch-export\""));
        assert!(registry.has_tool("mcp::local-mcp::image-batch-export"));
    }

    #[tokio::test]
    async fn forged_tool_renders_command_payload() {
        let registry = ToolRegistry::from_config(&ToolsConfig {
            builtin: vec!["read_file".into()],
            allow_shell: false,
            mcp_servers: vec!["local-mcp".into()],
        });
        registry
            .execute(
                "cli::forge_mcp_tool",
                r#"{
                    "server":"local-mcp",
                    "capability_name":"diagram export",
                    "purpose":"Export diagrams through a stable CLI wrapper",
                    "executable":"diagram-cli",
                    "subcommands":["export"],
                    "arguments":[
                        {"name":"project","description":"project path","required":true},
                        {"name":"theme","description":"theme name","required":false}
                    ],
                    "output_mode":"json"
                }"#,
            )
            .await
            .expect("forge tool");

        let result = registry
            .execute(
                "mcp::local-mcp::diagram-export",
                r#"{"project":"D:/demo/project.drawio","theme":"clean light"}"#,
            )
            .await
            .expect("execute forged tool");

        assert!(result.content.contains("\"delegate_tool\": \"mcp::local-mcp::invoke\""));
        assert!(result.content.contains("diagram-cli export"));
        assert!(result.content.contains("--project D:/demo/project.drawio"));
    }

    #[tokio::test]
    async fn forged_tools_persist_and_restore_from_spacetimedb() {
        let db = SpacetimeDb::from_config(&SpacetimeDbConfig {
            enabled: true,
            backend: SpacetimeBackend::InMemory,
            uri: "http://spacetimedb:3000".into(),
            module_name: "autoloop_core".into(),
            namespace: "autoloop".into(),
            pool_size: 4,
        });
        let registry = ToolRegistry::from_config(&ToolsConfig {
            builtin: vec!["read_file".into()],
            allow_shell: false,
            mcp_servers: vec!["local-mcp".into()],
        });
        registry.attach_spacetimedb(db.clone());

        registry
            .execute(
                "cli::forge_mcp_tool",
                r#"{
                    "server":"local-mcp",
                    "capability_name":"session replay",
                    "purpose":"Rebuild session-level CLI tooling from persisted manifests",
                    "executable":"autoloop-cli",
                    "subcommands":["session","replay"],
                    "arguments":[
                        {"name":"session","description":"session id","required":true}
                    ],
                    "output_mode":"json"
                }"#,
            )
            .await
            .expect("forge and persist");

        let recovered = ToolRegistry::from_config(&ToolsConfig {
            builtin: vec!["read_file".into()],
            allow_shell: false,
            mcp_servers: vec!["local-mcp".into()],
        });
        recovered.attach_spacetimedb(db);
        let restored = recovered
            .restore_persisted_manifests()
            .await
            .expect("restore");

        assert_eq!(restored, 1);
        assert!(recovered.has_tool("mcp::local-mcp::session-replay"));
        assert_eq!(recovered.manifests().len(), 1);
    }

    #[tokio::test]
    async fn governance_tools_change_capability_execution_state() {
        let registry = ToolRegistry::from_config(&ToolsConfig {
            builtin: vec!["read_file".into()],
            allow_shell: false,
            mcp_servers: vec!["local-mcp".into()],
        });

        registry
            .execute(
                "cli::forge_mcp_tool",
                r#"{
                    "server":"local-mcp",
                    "capability_name":"network deploy",
                    "purpose":"Deploy over network to remote target",
                    "executable":"deploy-cli",
                    "arguments":[{"name":"target","description":"host","required":true}],
                    "output_mode":"json"
                }"#,
            )
            .await
            .expect("forge");

        assert!(registry
            .execute("mcp::local-mcp::network-deploy", r#"{"target":"prod"}"#)
            .await
            .is_err());

        registry
            .execute(
                "cli::verify_capability",
                r#"{"tool_name":"mcp::local-mcp::network-deploy"}"#,
            )
            .await
            .expect("verify");
        assert!(registry
            .execute("mcp::local-mcp::network-deploy", r#"{"target":"prod"}"#)
            .await
            .is_ok());

        registry
            .execute(
                "cli::deprecate_capability",
                r#"{"tool_name":"mcp::local-mcp::network-deploy","health_score":0.2}"#,
            )
            .await
            .expect("deprecate");
        assert!(registry
            .execute("mcp::local-mcp::network-deploy", r#"{"target":"prod"}"#)
            .await
            .is_err());
    }
}

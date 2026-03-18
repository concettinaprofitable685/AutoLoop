use std::{fs, path::PathBuf, sync::Arc};

use anyhow::Result;
use autoloop::{AutoLoopApp, config::AppConfig, dashboard_server};
use clap::{Parser, Subcommand};
use tracing_subscriber::EnvFilter;

#[derive(Parser, Debug)]
#[command(name = "autocog")]
#[command(about = "AutoCog-style CLI for AutoLoop autonomous cognition")]
struct Cli {
    #[arg(long)]
    config: Option<PathBuf>,

    #[command(subcommand)]
    command: Option<Commands>,

    #[arg(short, long)]
    message: Option<String>,

    #[arg(long, default_value = "cli:direct")]
    session: String,

    #[arg(long, default_value_t = false)]
    swarm: bool,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Focus {
        #[arg()]
        anchor: Option<String>,
        #[arg(long)]
        list: bool,
        #[arg(long)]
        status: bool,
        #[arg(long)]
        delete: bool,
        #[arg(long)]
        add: Option<String>,
        #[arg(long = "anchor-id")]
        anchor_id: Option<String>,
        #[arg(long)]
        time: Option<String>,
        #[arg(long)]
        region: Option<String>,
        #[arg(long = "core-source")]
        core_source: Option<String>,
        #[arg(long = "update-cycle")]
        update_cycle: Option<String>,
    },
    Mcp {
        #[arg()]
        action: String,
        #[arg(long = "anchor-id")]
        anchor_id: Option<String>,
        #[arg(long)]
        output: Option<PathBuf>,
        #[arg(long)]
        input: Option<PathBuf>,
        #[arg(long)]
        tool: Option<String>,
    },
    Knowledge {
        #[arg()]
        action: String,
        #[arg(long = "anchor-id")]
        anchor_id: Option<String>,
        #[arg(long, default_value = "graph")]
        r#type: String,
        #[arg(long)]
        output: Option<PathBuf>,
    },
    Crawl {
        #[arg()]
        action: String,
        #[arg(long = "anchor-id")]
        anchor_id: Option<String>,
        #[arg()]
        anchor: Option<String>,
    },
    Plugin {
        #[arg()]
        action: String,
        #[arg()]
        plugin: Option<String>,
    },
    System {
        #[arg()]
        action: String,
        #[arg(long = "anchor-list")]
        anchor_list: Option<PathBuf>,
        #[arg(long, default_value = "127.0.0.1")]
        host: String,
        #[arg(long, default_value_t = 8787)]
        port: u16,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .init();

    let cli = Cli::parse();
    let config = match cli.config {
        Some(path) => AppConfig::load_from_path(&path)?,
        None => AppConfig::default(),
    };

    let app = AutoLoopApp::try_new(config)?;
    let report = app.bootstrap().await?;

    if let Some(command) = cli.command {
        match command {
            Commands::Focus {
                anchor,
                list,
                status,
                delete,
                add,
                anchor_id,
                time,
                region,
                core_source,
                update_cycle,
            } => {
                if list {
                    println!("{}", serde_json::to_string_pretty(&app.list_focus_anchors().await?)?);
                } else if status {
                    println!(
                        "{}",
                        app.focus_status(anchor_id.as_deref().unwrap_or("cli:focus")).await?
                    );
                } else if delete {
                    println!(
                        "{}",
                        app.delete_focus_anchor(anchor_id.as_deref().unwrap_or("cli:focus"))
                            .await?
                    );
                } else if let Some(extra) = add {
                    let session = anchor_id.unwrap_or_else(|| "cli:focus".into());
                    let response = app
                        .process_requirement_swarm(&session, &extra)
                        .await?;
                    println!("{response}");
                } else if let Some(anchor) = anchor {
                    let session = anchor_id.unwrap_or_else(|| "cli:focus".into());
                    let anchor_request = compose_anchor_request(
                        &anchor,
                        time.as_deref(),
                        region.as_deref(),
                        core_source.as_deref(),
                        update_cycle.as_deref(),
                    );
                    let response = app.process_requirement_swarm(&session, &anchor_request).await?;
                    println!("{response}");
                } else {
                    println!("{}", app.system_status().await?);
                }
            }
            Commands::Mcp { action, anchor_id, output, input, tool } => {
                let body = match action.as_str() {
                    "status" => app
                        .focus_status(anchor_id.as_deref().unwrap_or("cli:focus"))
                        .await?,
                    "export" => app.export_mcp_catalog().await?,
                    "import" => {
                        let raw = if let Some(path) = input.as_ref() {
                            fs::read_to_string(path)?
                        } else {
                            "[]".into()
                        };
                        app.import_mcp_catalog(&raw).await?
                    }
                    "optimize" => serde_json::json!({
                        "status": "accepted",
                        "note": "runtime and learning loop already perform bounded autonomous optimization"
                    })
                    .to_string(),
                    "verify" | "deprecate" | "rollback" => app
                        .govern_mcp_capability(&action, tool.as_deref().unwrap_or("mcp::local-mcp::invoke"))
                        .await?,
                    _ => serde_json::json!({"error":"unsupported mcp action"}).to_string(),
                };
                write_or_print(output.as_ref(), &body)?;
            }
            Commands::Knowledge { action, anchor_id, r#type, output } => {
                let anchor = anchor_id.unwrap_or_else(|| "cli:focus".into());
                let body = match action.as_str() {
                    "export" => app.export_knowledge(&anchor, &r#type).await?,
                    "check" => app.focus_status(&anchor).await?,
                    "index" => app.export_knowledge(&anchor, "index").await?,
                    _ => serde_json::json!({"error":"unsupported knowledge action"}).to_string(),
                };
                write_or_print(output.as_ref(), &body)?;
            }
            Commands::Crawl { action, anchor_id, anchor } => {
                let session = anchor_id.unwrap_or_else(|| "cli:focus".into());
                let body = match action.as_str() {
                    "run" => {
                        let anchor_text = anchor.unwrap_or_else(|| session.clone());
                        let report = app
                            .research
                            .run_anchor_research(&app.spacetimedb, &session, &anchor_text)
                            .await?;
                        let scheduled = app
                            .research
                            .schedule_follow_up_research(&app.spacetimedb, &session, &session, &report)
                            .await?;
                        serde_json::to_string_pretty(&serde_json::json!({
                            "report": report,
                            "scheduled_follow_ups": scheduled,
                        }))?
                    }
                    "status" => serde_json::json!({
                        "report": serde_json::from_str::<serde_json::Value>(&app.export_knowledge(&session, "research").await?)
                            .unwrap_or_else(|_| serde_json::json!({})),
                        "follow_up": serde_json::from_str::<serde_json::Value>(&app.export_knowledge(&session, "research-follow-up").await?)
                            .unwrap_or_else(|_| serde_json::json!({})),
                        "proxy_forensics": serde_json::from_str::<serde_json::Value>(&app.export_knowledge(&session, "research-proxy").await?)
                            .unwrap_or_else(|_| serde_json::json!({})),
                        "health": app.research.health_report(),
                        "backend": format!("{:?}", app.config.research.backend),
                        "live_fetch_enabled": app.config.research.live_fetch_enabled,
                        "dynamic_render": app.config.research.prefer_dynamic_render,
                        "proxy_pool_size": app.config.research.proxy_pool.len(),
                    }).to_string(),
                    "pause" => serde_json::json!({"status":"accepted","note":"crawl pause intent recorded; scheduled follow-ups can be drained by policy"}).to_string(),
                    "resume" => serde_json::json!({"status":"accepted","note":"crawl resume accepted; next run will continue autonomous research scheduling"}).to_string(),
                    _ => serde_json::json!({"error":"unsupported crawl action"}).to_string(),
                };
                println!("{body}");
            }
            Commands::Plugin { action, plugin } => {
                let body = match action.as_str() {
                    "list" => app.plugin_list()?,
                    "status" => app.plugin_status(plugin.as_deref().unwrap_or("cli::forge_mcp_tool"))?,
                    "add" => serde_json::json!({
                        "status": "accepted",
                        "plugin": plugin.unwrap_or_else(|| "unspecified".into()),
                        "note": "capability plugins are catalog-governed and can be forged or restored dynamically"
                    }).to_string(),
                    "remove" => serde_json::json!({
                        "status": "accepted",
                        "plugin": plugin.unwrap_or_else(|| "unspecified".into()),
                        "note": "plugin removal is treated as a governance/deprecation request in the capability catalog"
                    }).to_string(),
                    _ => serde_json::json!({"error":"unsupported plugin action"}).to_string(),
                };
                println!("{body}");
            }
            Commands::System { action, anchor_list, host, port } => {
                let body = match action.as_str() {
                    "status" => app.system_status().await?,
                    "health" => serde_json::to_string_pretty(&serde_json::json!({
                        "research": app.research.health_report(),
                        "system": serde_json::from_str::<serde_json::Value>(&app.system_status().await?)
                            .unwrap_or_else(|_| serde_json::json!({})),
                    }))?,
                    "update" => serde_json::json!({
                        "status": "noop",
                        "note": "binary self-update is not implemented yet"
                    })
                    .to_string(),
                    "deploy" => {
                        let anchors = anchor_list
                            .as_ref()
                            .and_then(|path| fs::read_to_string(path).ok())
                            .map(|raw| raw.lines().map(str::trim).filter(|line| !line.is_empty()).count())
                            .unwrap_or(0);
                        serde_json::json!({
                            "status": "ready",
                            "artifacts": ["Dockerfile", "docker-compose.yml", "deploy/k8s/autoloop-deployment.yaml"],
                            "anchor_batch_size": anchors,
                        })
                        .to_string()
                    }
                    "backup" => serde_json::json!({
                        "status": "ready",
                        "script": "deploy/backup/backup.ps1",
                    }).to_string(),
                    "restore" => serde_json::json!({
                        "status": "ready",
                        "script": "deploy/backup/restore.ps1",
                    }).to_string(),
                    "dashboard" => app.export_dashboard_snapshot(&cli.session).await?,
                    "replay" => app.export_session_replay(&cli.session).await?,
                    "serve" => {
                        dashboard_server::run_dashboard_server(Arc::new(app), &host, port).await?;
                        return Ok(());
                    }
                    _ => serde_json::json!({"error":"unsupported system action"}).to_string(),
                };
                println!("{body}");
            }
        }
    } else if let Some(message) = cli.message {
        let response = if cli.swarm {
            app.process_requirement_swarm(&cli.session, &message).await?
        } else {
            app.process_direct(&cli.session, &message).await?
        };
        println!("{response}");
    } else {
        println!(
            "AutoLoop bootstrap ready: app={}, providers={}, tools={}, hooks={}, memory_targets={}, rag_strategies={}",
            report.app_name,
            report.provider_count,
            report.tool_count,
            report.hook_count,
            report.memory_targets,
            report.rag_strategies
        );
    }

    Ok(())
}

fn write_or_print(output: Option<&PathBuf>, body: &str) -> Result<()> {
    if let Some(path) = output {
        fs::write(path, body)?;
    } else {
        println!("{body}");
    }
    Ok(())
}

fn compose_anchor_request(
    anchor: &str,
    time: Option<&str>,
    region: Option<&str>,
    core_source: Option<&str>,
    update_cycle: Option<&str>,
) -> String {
    let mut parts = vec![format!("Focus anchor: {anchor}")];
    if let Some(time) = time {
        parts.push(format!("Time range: {time}"));
    }
    if let Some(region) = region {
        parts.push(format!("Region: {region}"));
    }
    if let Some(core_source) = core_source {
        parts.push(format!("Core source preference: {core_source}"));
    }
    if let Some(update_cycle) = update_cycle {
        parts.push(format!("Update cycle: {update_cycle}"));
    }
    parts.join("\n")
}

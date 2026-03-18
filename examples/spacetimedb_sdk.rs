use autoloop::{
    config::AppConfig,
    runtime::McpDispatchRequest,
    AutoLoopApp,
};
use autoloop_spacetimedb_adapter::PermissionAction;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let app = AutoLoopApp::new(AppConfig::default());
    app.bootstrap().await?;

    app.spacetimedb
        .grant_permissions(
            "scheduler",
            vec![
                PermissionAction::Read,
                PermissionAction::Write,
                PermissionAction::Dispatch,
            ],
        )
        .await?;

    let event = app
        .runtime
        .dispatch_mcp_event(
            &app.spacetimedb,
            McpDispatchRequest {
                session_id: "example-session".into(),
                tool_name: "mcp::local-mcp::invoke".into(),
                payload: "{\"action\":\"sync-memory\"}".into(),
                actor_id: "scheduler".into(),
            },
        )
        .await?;

    let reply = app
        .process_direct("example-session", "Summarize how SpacetimeDB stores scheduler state.")
        .await?;

    app.spacetimedb
        .upsert_agent_state(
            "example-session".into(),
            "Summarize how SpacetimeDB stores scheduler state.".into(),
            Some(reply.clone()),
        )
        .await?;

    app.spacetimedb
        .upsert_knowledge(
            "scheduler:summary".into(),
            reply,
            "example".into(),
        )
        .await?;
    app.spacetimedb.update_schedule_status(event.id, "completed").await?;

    println!("SpacetimeDB SDK example completed for event {}", event.id);
    Ok(())
}

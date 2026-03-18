use serde::{Deserialize, Serialize};
use spacetimedb::{DbContext, ReducerContext, Table};

#[derive(Clone, Debug, Serialize, Deserialize, spacetimedb::SpacetimeType)]
pub enum PermissionAction {
    Read,
    Write,
    Dispatch,
    Admin,
}

#[derive(Clone, Debug, Serialize, Deserialize, spacetimedb::SpacetimeType)]
pub enum LearningEventKind {
    Failure,
    Success,
    ToolCall,
    RouteDecision,
    Audit,
}

#[spacetimedb::table(accessor = permission_grant, public)]
pub struct PermissionGrant {
    #[primary_key]
    pub actor_id: String,
    pub permissions: Vec<PermissionAction>,
}

#[spacetimedb::table(accessor = schedule_event, public)]
pub struct ScheduleEvent {
    #[primary_key]
    pub id: u64,
    pub session_id: String,
    pub topic: String,
    pub tool_name: String,
    pub payload: String,
    pub actor_id: String,
    pub status: String,
}

#[spacetimedb::table(accessor = agent_state, public)]
pub struct AgentState {
    #[primary_key]
    pub session_id: String,
    pub last_user_message: String,
    pub last_assistant_message: Option<String>,
}

#[spacetimedb::table(accessor = knowledge_record, public)]
pub struct KnowledgeRecord {
    #[primary_key]
    pub key: String,
    pub value: String,
    pub source: String,
}

#[spacetimedb::table(accessor = reflexion_episode, public)]
pub struct ReflexionEpisode {
    #[primary_key]
    pub id: String,
    pub session_id: String,
    pub objective: String,
    pub hypothesis: String,
    pub outcome: String,
    pub lesson: String,
    pub status: String,
    pub score: f32,
    pub created_at_ms: u64,
}

#[spacetimedb::table(accessor = skill_library_record, public)]
pub struct SkillLibraryRecord {
    #[primary_key]
    pub id: String,
    pub session_id: String,
    pub name: String,
    pub trigger: String,
    pub procedure: String,
    pub confidence: f32,
    pub success_rate: f32,
    pub evidence_count: u32,
    pub created_at_ms: u64,
    pub updated_at_ms: u64,
}

#[spacetimedb::table(accessor = causal_edge_record, public)]
pub struct CausalEdgeRecord {
    #[primary_key]
    pub id: String,
    pub session_id: String,
    pub cause: String,
    pub effect: String,
    pub evidence: String,
    pub strength: f32,
    pub confidence: f32,
    pub created_at_ms: u64,
}

#[spacetimedb::table(accessor = learning_session_record, public)]
pub struct LearningSessionRecord {
    #[primary_key]
    pub id: String,
    pub session_id: String,
    pub objective: String,
    pub status: String,
    pub priority: f32,
    pub summary: String,
    pub started_at_ms: u64,
    pub completed_at_ms: Option<u64>,
}

#[spacetimedb::table(accessor = witness_log_record, public)]
pub struct WitnessLogRecord {
    #[primary_key]
    pub id: String,
    pub session_id: String,
    pub event_type: LearningEventKind,
    pub source: String,
    pub detail: String,
    pub score: f32,
    pub created_at_ms: u64,
    pub metadata_json: String,
}

#[spacetimedb::reducer(init)]
pub fn init(_ctx: &ReducerContext) {}

#[spacetimedb::reducer]
pub fn grant_permissions(
    ctx: &ReducerContext,
    actor_id: String,
    permissions: Vec<PermissionAction>,
) -> Result<(), String> {
    let row = PermissionGrant {
        actor_id: actor_id.clone(),
        permissions,
    };

    if ctx.db().permission_grant().actor_id().find(&actor_id).is_some() {
        ctx.db().permission_grant().actor_id().update(row);
    } else {
        ctx.db().permission_grant().insert(row);
    }

    Ok(())
}

#[spacetimedb::reducer]
pub fn create_schedule_event(
    ctx: &ReducerContext,
    id: u64,
    session_id: String,
    topic: String,
    tool_name: String,
    payload: String,
    actor_id: String,
    status: String,
) -> Result<(), String> {
    if ctx.db().schedule_event().id().find(&id).is_some() {
        return Err(format!("schedule event {id} already exists"));
    }

    ctx.db().schedule_event().insert(ScheduleEvent {
        id,
        session_id,
        topic,
        tool_name,
        payload,
        actor_id,
        status,
    });

    Ok(())
}

#[spacetimedb::reducer]
pub fn update_schedule_status(
    ctx: &ReducerContext,
    id: u64,
    status: String,
) -> Result<(), String> {
    let mut row = ctx
        .db()
        .schedule_event()
        .id()
        .find(&id)
        .ok_or_else(|| format!("schedule event {id} not found"))?;

    row.status = status;
    ctx.db().schedule_event().id().update(row);

    Ok(())
}

#[spacetimedb::reducer]
pub fn upsert_agent_state(
    ctx: &ReducerContext,
    session_id: String,
    last_user_message: String,
    last_assistant_message: Option<String>,
) -> Result<(), String> {
    let row = AgentState {
        session_id: session_id.clone(),
        last_user_message,
        last_assistant_message,
    };

    if ctx.db().agent_state().session_id().find(&session_id).is_some() {
        ctx.db().agent_state().session_id().update(row);
    } else {
        ctx.db().agent_state().insert(row);
    }

    Ok(())
}

#[spacetimedb::reducer]
pub fn upsert_knowledge(
    ctx: &ReducerContext,
    key: String,
    value: String,
    source: String,
) -> Result<(), String> {
    let row = KnowledgeRecord {
        key: key.clone(),
        value,
        source,
    };

    if ctx.db().knowledge_record().key().find(&key).is_some() {
        ctx.db().knowledge_record().key().update(row);
    } else {
        ctx.db().knowledge_record().insert(row);
    }

    Ok(())
}

#[spacetimedb::reducer]
pub fn upsert_reflexion_episode(
    ctx: &ReducerContext,
    id: String,
    session_id: String,
    objective: String,
    hypothesis: String,
    outcome: String,
    lesson: String,
    status: String,
    score: f32,
    created_at_ms: u64,
) -> Result<(), String> {
    let row = ReflexionEpisode {
        id: id.clone(),
        session_id,
        objective,
        hypothesis,
        outcome,
        lesson,
        status,
        score,
        created_at_ms,
    };

    if ctx.db().reflexion_episode().id().find(&id).is_some() {
        ctx.db().reflexion_episode().id().update(row);
    } else {
        ctx.db().reflexion_episode().insert(row);
    }

    Ok(())
}

#[spacetimedb::reducer]
pub fn upsert_skill_library_record(
    ctx: &ReducerContext,
    id: String,
    session_id: String,
    name: String,
    trigger: String,
    procedure: String,
    confidence: f32,
    success_rate: f32,
    evidence_count: u32,
    created_at_ms: u64,
    updated_at_ms: u64,
) -> Result<(), String> {
    let row = SkillLibraryRecord {
        id: id.clone(),
        session_id,
        name,
        trigger,
        procedure,
        confidence,
        success_rate,
        evidence_count,
        created_at_ms,
        updated_at_ms,
    };

    if ctx.db().skill_library_record().id().find(&id).is_some() {
        ctx.db().skill_library_record().id().update(row);
    } else {
        ctx.db().skill_library_record().insert(row);
    }

    Ok(())
}

#[spacetimedb::reducer]
pub fn upsert_causal_edge_record(
    ctx: &ReducerContext,
    id: String,
    session_id: String,
    cause: String,
    effect: String,
    evidence: String,
    strength: f32,
    confidence: f32,
    created_at_ms: u64,
) -> Result<(), String> {
    let row = CausalEdgeRecord {
        id: id.clone(),
        session_id,
        cause,
        effect,
        evidence,
        strength,
        confidence,
        created_at_ms,
    };

    if ctx.db().causal_edge_record().id().find(&id).is_some() {
        ctx.db().causal_edge_record().id().update(row);
    } else {
        ctx.db().causal_edge_record().insert(row);
    }

    Ok(())
}

#[spacetimedb::reducer]
pub fn upsert_learning_session_record(
    ctx: &ReducerContext,
    id: String,
    session_id: String,
    objective: String,
    status: String,
    priority: f32,
    summary: String,
    started_at_ms: u64,
    completed_at_ms: Option<u64>,
) -> Result<(), String> {
    let row = LearningSessionRecord {
        id: id.clone(),
        session_id,
        objective,
        status,
        priority,
        summary,
        started_at_ms,
        completed_at_ms,
    };

    if ctx.db().learning_session_record().id().find(&id).is_some() {
        ctx.db().learning_session_record().id().update(row);
    } else {
        ctx.db().learning_session_record().insert(row);
    }

    Ok(())
}

#[spacetimedb::reducer]
pub fn append_witness_log_record(
    ctx: &ReducerContext,
    id: String,
    session_id: String,
    event_type: LearningEventKind,
    source: String,
    detail: String,
    score: f32,
    created_at_ms: u64,
    metadata_json: String,
) -> Result<(), String> {
    let row = WitnessLogRecord {
        id: id.clone(),
        session_id,
        event_type,
        source,
        detail,
        score,
        created_at_ms,
        metadata_json,
    };

    if ctx.db().witness_log_record().id().find(&id).is_some() {
        ctx.db().witness_log_record().id().update(row);
    } else {
        ctx.db().witness_log_record().insert(row);
    }

    Ok(())
}

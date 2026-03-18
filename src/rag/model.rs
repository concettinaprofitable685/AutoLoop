use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum DocumentStatus {
    Pending,
    Chunked,
    GraphReady,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocumentRecord {
    pub id: u64,
    pub title: String,
    pub source_uri: String,
    pub raw_text: String,
    pub status: DocumentStatus,
    pub created_at_ms: u64,
    pub chunk_count: u32,
    pub entity_count: u32,
    pub relationship_count: u32,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChunkRecord {
    pub id: u64,
    pub document_id: u64,
    pub ordinal: u32,
    pub text: String,
    pub token_count: u32,
    pub offset_start: u32,
    pub offset_end: u32,
    pub previous_chunk_id: Option<u64>,
    pub next_chunk_id: Option<u64>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct EntityRecord {
    pub id: u64,
    pub canonical_name: String,
    pub normalized_name: String,
    pub entity_type: String,
    pub description: String,
    pub salience: u32,
    pub mention_count: u32,
    pub degree: u32,
    pub weight: u32,
    pub first_document_id: u64,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct MentionRecord {
    pub id: u64,
    pub document_id: u64,
    pub chunk_id: u64,
    pub entity_id: u64,
    pub surface: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RelationshipRecord {
    pub id: u64,
    pub document_id: u64,
    pub source_entity_id: u64,
    pub target_entity_id: u64,
    pub relation_type: String,
    pub weight: u32,
    pub confidence: u32,
    pub evidence_chunk_ids: Vec<u64>,
    pub description: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommunityRecord {
    pub id: u64,
    pub document_id: u64,
    pub label: String,
    pub member_entity_ids: Vec<u64>,
    pub relationship_ids: Vec<u64>,
    pub rank: u32,
    pub summary: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ExtractedEntity {
    pub name: String,
    pub entity_type: String,
    pub description: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ExtractedRelationship {
    pub source_name: String,
    pub target_name: String,
    pub relation_type: String,
    pub description: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ExtractedChunkGraph {
    pub entities: Vec<ExtractedEntity>,
    pub relationships: Vec<ExtractedRelationship>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct IngestDocumentResult {
    pub document_id: u64,
    pub chunk_ids: Vec<u64>,
    pub entity_ids: Vec<u64>,
    pub relationship_ids: Vec<u64>,
    pub community_ids: Vec<u64>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct QueryContext {
    pub document_id: u64,
    pub mode: QueryMode,
    pub query: String,
    pub matched_chunk_ids: Vec<u64>,
    pub matched_entity_ids: Vec<u64>,
    pub matched_relationship_ids: Vec<u64>,
    pub summary: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum QueryMode {
    Local,
    Global,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CommunityBuildResult {
    pub document_id: u64,
    pub community_ids: Vec<u64>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ChunkFactResult {
    pub chunk_id: u64,
    pub entity_ids: Vec<u64>,
    pub relationship_ids: Vec<u64>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SearchHit {
    pub chunk_id: u64,
    pub entity_ids: BTreeSet<u64>,
    pub score: u32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RankedRelationship {
    pub relationship_id: u64,
    pub weight: u32,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct DatabaseSnapshot {
    pub documents: Vec<DocumentRecord>,
    pub chunks: Vec<ChunkRecord>,
    pub entities: Vec<EntityRecord>,
    pub mentions: Vec<MentionRecord>,
    pub relationships: Vec<RelationshipRecord>,
    pub communities: Vec<CommunityRecord>,
}

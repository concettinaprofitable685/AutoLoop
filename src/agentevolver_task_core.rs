use serde::{Deserialize, Serialize};

use crate::adaptive_framework::{PromptTemplateAsset, PromptTemplateBundle};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentEvolverTaskPack {
    pub system_prompt: Vec<String>,
    pub routing_prompt: Vec<String>,
    pub tool_prompt: Vec<String>,
    pub forge_prompt: Vec<String>,
}

impl AgentEvolverTaskPack {
    pub fn from_bundle(bundle: &PromptTemplateBundle) -> Self {
        Self {
            system_prompt: flatten_assets(&bundle.system_templates),
            routing_prompt: flatten_assets(&bundle.routing_templates),
            tool_prompt: flatten_assets(&bundle.tool_templates),
            forge_prompt: flatten_assets(&bundle.forge_templates),
        }
    }
}

fn flatten_assets(assets: &[PromptTemplateAsset]) -> Vec<String> {
    assets
        .iter()
        .flat_map(|asset| asset.instructions.clone())
        .collect()
}

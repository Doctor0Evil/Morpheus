use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrchestratorConfig {
    pub spec_path: PathBuf,
}

impl OrchestratorConfig {
    pub fn from_env_or_default() -> Self {
        let spec_path = std::env::var("MORPHEUS_SPEC_PATH")
            .map(PathBuf::from)
            .unwrap_or_else(|_| PathBuf::from("morpheus-spec.aln"));
        Self { spec_path }
    }
}

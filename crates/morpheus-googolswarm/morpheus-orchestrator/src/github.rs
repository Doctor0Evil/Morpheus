use crate::events::CiEvent;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitHubInput {
    pub repository: String,
    pub commit: String,
    pub workflow: String,
}

impl From<GitHubInput> for CiEvent {
    fn from(v: GitHubInput) -> Self {
        CiEvent {
            repository: v.repository,
            commit: v.commit,
            workflow: v.workflow,
        }
    }
}

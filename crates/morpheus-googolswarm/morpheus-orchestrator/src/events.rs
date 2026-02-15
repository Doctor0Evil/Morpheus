use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CiEvent {
    pub repository: String,
    pub commit: String,
    pub workflow: String,
}

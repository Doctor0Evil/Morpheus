use std::fs;
use std::path::PathBuf;

use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EcoNode {
    pub node_id: String,
    pub contaminant: String,
    pub k_n: f64,
    pub ecoimpact_score: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EcoShard {
    pub generated_at: String,
    pub nodes: Vec<EcoNode>,
}

pub fn load_latest_shard(dir: &str) -> Result<Option<EcoShard>> {
    let mut entries: Vec<_> = fs::read_dir(dir)?
        .filter_map(|e| e.ok())
        .collect();
    entries.sort_by_key(|e| e.file_name());
    if let Some(last) = entries.last() {
        let mut path = PathBuf::from(dir);
        path.push(last.file_name());
        let raw = fs::read_to_string(path)?;
        let shard: EcoShard = serde_json::from_str(&raw)?;
        Ok(Some(shard))
    } else {
        Ok(None)
    }
}

pub fn band_for_score(k_n_norm: f64) -> f64 {
    if k_n_norm < 0.3 {
        0.1
    } else if k_n_norm < 0.7 {
        0.5
    } else {
        0.9
    }
}

use std::fs;
use std::path::PathBuf;

use anyhow::Result;
use chrono::Utc;
use serde::Serialize;

use crate::state::CeimNodeState;

#[derive(Serialize)]
pub struct CeimShard {
    pub generated_at: String,
    pub nodes: Vec<CeimNodeState>,
}

pub fn write_shard(output_dir: &str, nodes: Vec<CeimNodeState>) -> Result<()> {
    let shard = CeimShard {
        generated_at: Utc::now().to_rfc3339(),
        nodes,
    };
    let json = serde_json::to_string_pretty(&shard)?;
    let mut path = PathBuf::from(output_dir);
    fs::create_dir_all(&path)?;
    path.push(format!("ceim_shard_{}.json", shard.generated_at.replace(':', "_")));
    fs::write(path, json)?;
    Ok(())
}

mod config;
mod feeds;
mod state;
mod shards;

use std::time::Duration;

use anyhow::Result;
use chrono::Utc;
use tracing::{error, info};
use tracing_subscriber::EnvFilter;

use ceim-kernel::{CeimKernel, RegulatoryLimits, TimeSample};

use config::Config;
use feeds::fetch_samples;
use shards::write_shard;
use state::CeimNodeState;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let cfg = load_config()?;
    loop {
        if let Err(e) = tick(&cfg).await {
            error!("tick error: {e:?}");
        }
        tokio::time::sleep(Duration::from_secs(cfg.poll_interval_seconds)).await;
    }
}

fn load_config() -> Result<Config> {
    let raw = std::fs::read_to_string("phoenix-bridge-config.json")?;
    let cfg: Config = serde_json::from_str(&raw)?;
    Ok(cfg)
}

async fn tick(cfg: &Config) -> Result<()> {
    info!("fetching water samples");
    let samples = fetch_samples(&cfg.water_quality_feed_url).await?;
    let mut nodes = Vec::new();

    let groups = group_by_node_and_contaminant(samples);
    for ((node_id, contaminant), s) in groups {
        let times: Vec<TimeSample> = s
            .iter()
            .enumerate()
            .map(|(i, sm)| TimeSample {
                t_hours: i as f64,
                c_in: sm.c_in,
                c_out: sm.c_out,
                flow_q: sm.flow_q,
            })
            .collect();

        let limits = RegulatoryLimits {
            epa: Some(1.0),
            eu: Some(1.0),
            who: Some(1.0),
        };
        let impact = CeimKernel::compute(&contaminant, 1.0, &times, &limits);
        nodes.push(CeimNodeState {
            node_id: node_id.clone(),
            contaminant: contaminant.clone(),
            k_n: impact.k_n,
            last_updated: Utc::now(),
        });
    }

    write_shard(&cfg.output_dir, nodes)?;
    Ok(())
}

use std::collections::HashMap;
use feeds::WaterSample;

fn group_by_node_and_contaminant(
    samples: Vec<WaterSample>,
) -> HashMap<(String, String), Vec<WaterSample>> {
    let mut map: HashMap<(String, String), Vec<WaterSample>> = HashMap::new();
    for s in samples {
        map.entry((s.node_id.clone(), s.contaminant.clone()))
            .or_default()
            .push(s);
    }
    map
}

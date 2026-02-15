use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct WaterSample {
    pub timestamp: DateTime<Utc>,
    pub node_id: String,
    pub contaminant: String,
    pub c_in: f64,
    pub c_out: f64,
    pub flow_q: f64,
}

pub async fn fetch_samples(feed_url: &str) -> Result<Vec<WaterSample>> {
    let resp = reqwest::get(feed_url).await?;
    let samples: Vec<WaterSample> = resp.json().await?;
    Ok(samples)
}

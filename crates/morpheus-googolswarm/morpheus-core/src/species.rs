use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SpeciesKind {
    Human,
    Synthetic,
    Hybrid,
    Other(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiophysicalEnvelope {
    pub species: SpeciesKind,
    pub min_safe_roh: f32,
    pub max_safe_roh: f32,
    pub roH_monotone: bool,
    pub no_cross_species_signals: bool,
}

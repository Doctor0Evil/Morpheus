use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ViabilityState {
    pub load_fraction: f64,
    pub vibration_index: f64,
    pub temperature_c: f64,
}

pub struct ViabilityKernel;

impl ViabilityKernel {
    pub fn is_within_envelope(state: &ViabilityState) -> bool {
        state.load_fraction <= 1.0
            && state.vibration_index <= 1.0
            && (0.0..=60.0).contains(&state.temperature_c)
    }
}

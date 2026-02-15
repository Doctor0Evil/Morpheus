use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LyapunovResidual {
    pub v_k: f64,
}

impl LyapunovResidual {
    pub fn from_state(deviation: f64) -> Self {
        let v_k = deviation * deviation;
        Self { v_k }
    }

    pub fn decreasing(&self, next: &LyapunovResidual) -> bool {
        next.v_k <= self.v_k
    }
}

use serde::{Deserialize, Serialize};

use crate::{mass_load, RegulatoryLimits, SupremeLimit};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CeimNodeImpact {
    pub contaminant: String,
    pub omega: f64,
    pub k_n: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TimeSample {
    pub t_hours: f64,
    pub c_in: f64,
    pub c_out: f64,
    pub flow_q: f64,
}

pub struct CeimKernel;

impl CeimKernel {
    pub fn compute(
        contaminant: &str,
        omega: f64,
        samples: &[TimeSample],
        limits: &RegulatoryLimits,
    ) -> CeimNodeImpact {
        let supreme = limits.supreme();
        let m_x = mass_load(samples);
        let mut k_n = 0.0;
        if supreme.value > 0.0 {
            k_n = omega * m_x / supreme.value;
        }
        CeimNodeImpact {
            contaminant: contaminant.to_string(),
            omega,
            k_n,
        }
    }
}

use crate::telemetry::ImplantTelemetry;

#[derive(Clone, Debug)]
pub struct CoherenceScore {
    pub value_0_1: f32,
    pub roh_estimate_0_1: f32,
    pub decay_estimate_0_1: f32,
}

pub trait ImplantInterfaceGuard {
    /// Non-actuating: computes scores and returns a log record.
    fn evaluate(&self, t: &ImplantTelemetry) -> CoherenceScore;
}

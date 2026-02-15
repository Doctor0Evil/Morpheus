use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum TrainType {
    Gac,
    Ix,
    Ro,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TreatmentDesign {
    pub name: String,
    pub train_type: TrainType,
    pub c_out: f64,
    pub energy_kwh: f64,
}

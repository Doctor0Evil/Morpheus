use anyhow::Result;
use ceim-kernel::{CeimKernel, RegulatoryLimits, TimeSample};
use cpvm-kernel::{ViabilityKernel, ViabilityState};

use crate::series::TimeSeriesPoint;

pub struct IntakePlan {
    pub start_hour: u32,
    pub end_hour: u32,
    pub k_n_tds: f64,
    pub k_n_nitrate: f64,
}

pub fn optimize(
    series: &[TimeSeriesPoint],
    viability: &ViabilityState,
) -> Result<Option<IntakePlan>> {
    if !ViabilityKernel::is_within_envelope(viability) {
        return Ok(None);
    }

    let mut best: Option<IntakePlan> = None;
    for window in series.windows(2) {
        let a = &window[0];
        let b = &window[1];

        let samples_tds = vec![
            TimeSample {
                t_hours: a.hour as f64,
                c_in: a.tds,
                c_out: 0.0,
                flow_q: a.flow_q,
            },
            TimeSample {
                t_hours: b.hour as f64,
                c_in: b.tds,
                c_out: 0.0,
                flow_q: b.flow_q,
            },
        ];
        let samples_nitrate = vec![
            TimeSample {
                t_hours: a.hour as f64,
                c_in: a.nitrate,
                c_out: 0.0,
                flow_q: a.flow_q,
            },
            TimeSample {
                t_hours: b.hour as f64,
                c_in: b.nitrate,
                c_out: 0.0,
                flow_q: b.flow_q,
            },
        ];
        let limits = RegulatoryLimits {
            epa: Some(1.0),
            eu: Some(1.0),
            who: Some(1.0),
        };
        let tds_impact = CeimKernel::compute("TDS", 1.0, &samples_tds, &limits);
        let nitrate_impact = CeimKernel::compute("nitrate", 1.0, &samples_nitrate, &limits);

        let candidate = IntakePlan {
            start_hour: a.hour,
            end_hour: b.hour,
            k_n_tds: tds_impact.k_n,
            k_n_nitrate: nitrate_impact.k_n,
        };

        let better = match &best {
            None => true,
            Some(bst) => {
                (candidate.k_n_tds + candidate.k_n_nitrate)
                    > (bst.k_n_tds + bst.k_n_nitrate)
            }
        };
        if better {
            best = Some(candidate);
        }
    }
    Ok(best)
}

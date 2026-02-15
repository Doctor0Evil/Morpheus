use anyhow::Result;

use ceim-kernel::{CeimKernel, RegulatoryLimits, TimeSample};

use crate::model::{Basin, RankedSchedule, ScheduleOption};

pub fn rank_schedules(
    basins: &[Basin],
    options: &[ScheduleOption],
    samples: &[TimeSample],
) -> Result<Vec<RankedSchedule>> {
    let mut results = Vec::new();
    for opt in options {
        if let Some(basin) = basins.iter().find(|b| b.id == opt.basin_id) {
            let impact = CeimKernel::compute(
                "nitrate",
                1.0,
                samples,
                &RegulatoryLimits {
                    epa: Some(1.0),
                    eu: Some(1.0),
                    who: Some(1.0),
                },
            );
            let k_n_per_kwh = impact.k_n / basin.energy_kwh_per_day.max(1.0);
            let k_n_per_hectare = impact.k_n / basin.area_ha.max(0.1);
            results.push(RankedSchedule {
                basin_id: basin.id.clone(),
                start_hour: opt.start_hour,
                end_hour: opt.end_hour,
                k_n_per_kwh,
                k_n_per_hectare,
            });
        }
    }
    results.sort_by(|a, b| b.k_n_per_kwh.partial_cmp(&a.k_n_per_kwh).unwrap());
    Ok(results)
}

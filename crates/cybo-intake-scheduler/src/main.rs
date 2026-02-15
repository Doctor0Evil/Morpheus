mod series;
mod optimizer;

use anyhow::Result;
use cpvm-kernel::ViabilityState;

use optimizer::optimize;
use series::TimeSeriesPoint;

fn main() -> Result<()> {
    let series = vec![
        TimeSeriesPoint {
            hour: 0,
            tds: 500.0,
            nitrate: 8.0,
            flow_q: 1.0,
        },
        TimeSeriesPoint {
            hour: 6,
            tds: 400.0,
            nitrate: 6.0,
            flow_q: 1.0,
        },
        TimeSeriesPoint {
            hour: 12,
            tds: 450.0,
            nitrate: 7.0,
            flow_q: 1.0,
        },
    ];

    let viability = ViabilityState {
        load_fraction: 0.7,
        vibration_index: 0.5,
        temperature_c: 30.0,
    };

    if let Some(plan) = optimize(&series, &viability)? {
        println!(
            "Intake {}-{}, K_n(TDS)={:.3}, K_n(nitrate)={:.3}",
            plan.start_hour, plan.end_hour, plan.k_n_tds, plan.k_n_nitrate
        );
    } else {
        println!("No viable intake window within CPVM envelope");
    }

    Ok(())
}

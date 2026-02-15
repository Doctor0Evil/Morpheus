use crate::TimeSample;

pub fn mass_load(samples: &[TimeSample]) -> f64 {
    if samples.is_empty() {
        return 0.0;
    }
    let mut total = 0.0;
    for w in samples.windows(2) {
        let a = &w[0];
        let b = &w[1];
        let dt = b.t_hours - a.t_hours;
        if dt <= 0.0 {
            continue;
        }
        let cin_minus_cout_a = a.c_in - a.c_out;
        let cin_minus_cout_b = b.c_in - b.c_out;
        let q_avg = 0.5 * (a.flow_q + b.flow_q);
        let integrand_avg = 0.5 * (cin_minus_cout_a + cin_minus_cout_b);
        total += integrand_avg * q_avg * dt;
    }
    total
}

///  \brief  Internal Numerical and Analytic Number Theory Algorithms
///  \author F. Erriques (Translated to Rust)

use common::common_data::Crash;

pub fn log_integral_co_chain(x: f64) -> f64 {
    if x <= 0.0 || (x - 1.0).abs() < 1.0e-20 {
        // Matches C++ behavior: returns 0.0 near singularities 
        return 0.0;
    }
    1.0 / x.ln()
}

pub fn factorial(par: u32) -> u64 {
    let mut res = 1_u64;
    for c in (2..=par as u64).rev() {
        res *= c;
    }
    res
}

pub fn exadecimal_converter_to(par: u32) -> String {
    let exa_figures = b"0123456789ABCDEF";
    let mut sb = String::from("0x");
    let mut abacus_left_most_position = ((par as f64).ln() / 16.0_f64.ln()).floor() as i32;
    let mut dividendo = par as f64;

    while abacus_left_most_position >= 0 {
        let divisor = 16.0_f64.powi(abacus_left_most_position);
        let quotient = (dividendo / divisor).floor();
        let resto = dividendo - quotient * divisor;
        sb.push(exa_figures[quotient as usize] as char);
        dividendo = resto;
        abacus_left_most_position -= 1;
    }
    sb
}

pub fn exp_integral_ei_ramanujan(x: f64) -> f64 {
    const EULER_GAMMA: f64 = 0.577216;
    let mut external_factor = 0.0;

    for n in 1..=99 {
        let mut internal_factor = 0.0;
        let limit = (((n as f64) - 1.0) / 2.0).floor() as i32;
        for k in 0..limit {
            internal_factor += 1.0 / (2.0 * k as f64 + 1.0);
        }

        let sign = if (n - 1) % 2 == 1 { -1.0 } else { 1.0 };
        let numerator = sign * x.powi(n);
        let denominator = (factorial(n as u32) as f64) * 2.0_f64.powi(n - 1);

        external_factor += (numerator / denominator) * internal_factor;
    }
    external_factor + EULER_GAMMA + x.ln()
}
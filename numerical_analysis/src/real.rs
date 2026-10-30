use common::common_data::{Couple, Domain, Crash, PI};
use crate::real_matrix::RealMatrix;

pub fn fabs(x: f64) -> f64 {
    if x < 0.0 { -x } else { x }
}

pub fn fact(x: usize) -> f64 {
    let mut res = 1.0;
    let mut i = x;
    while i > 1 {
        res *= i as f64;
        i -= 1;
    }
    res
}

pub fn is_odd(index: usize) -> bool {
    let real = index as f64 / 2.0;
    (real - (real as usize) as f64) > 1e-30
}

pub fn general_rounder(par: f64) -> f64 {
    const EPSILON: f64 = 1e-16;
    let is_negative = par < 0.0;
    let ipart = par as i64;
    let fpart = par - ipart as f64;
    if fabs(fpart) < EPSILON {
        ipart as f64
    } else if fabs(fpart) > (1.0 - EPSILON) {
        if is_negative {
            (ipart as f64) - 1.0
        } else {
            (ipart as f64) + 1.0
        }
    } else {
        par
    }
}

pub fn nat_pow(x: f64, n: usize) -> f64 {
    let mut res = 1.0;
    for _ in 0..n {
        res *= x;
    }
    res
}

pub fn pow_f64(base: f64, esp: f64) -> Result<f64, Domain> {
    if fabs(base) > 1e-50 {
        Ok(exp(esp * ln(base)?))
    } else {
        Err(1)
    }
}

pub fn ln(x: f64) -> Result<f64, Domain> {
    if x < 1e-60 {
        return Err(1);
    }
    let (admitted, is_right_side) = if x >= 1.0 {
        (1.09, true)
    } else {
        (0.9, false)
    };

    let mut res = 1.0;
    let mut c = 0.0;
    loop {
        res *= admitted;
        c += 1.0;
        if is_right_side {
            if res >= x { break; }
        } else {
            if res <= x { break; }
        }
    }
    let resto = x / res;
    res = 0.0;
    let mut bi_res = 0.0;
    const REDUCT_ORDER: usize = 80;
    for k in 0..REDUCT_ORDER {
        let sign = if is_odd(k) { -1.0 } else { 1.0 };
        let denom = (k + 1) as f64;
        res += sign / denom * nat_pow(admitted - 1.0, k + 1);
        bi_res += sign / denom * nat_pow(resto - 1.0, k + 1);
    }
    Ok(c * res + bi_res)
}

pub fn exp(x: f64) -> f64 {
    let mut val = x;
    let mut is_negative = false;
    if val < 0.0 {
        val *= -1.0;
        is_negative = true;
    }

    let admitted = 1.09;
    let mut res = 0.0;
    let mut c = 0.0;
    loop {
        res += admitted;
        c += 1.0;
        if res > val {
            res -= admitted;
            c -= 1.0;
            break;
        }
    }
    let resto = val - res;

    let order = 70;
    let mut the_fact = 1.0;
    let e_admitted = 2.9742740725630653163119065891359;
    let mut e_resto = 0.0;
    for k in 0..order {
        if k > 1 {
            the_fact *= k as f64;
        }
        e_resto += nat_pow(resto, k) / the_fact;
    }

    let mut e_admitted_alla_c = 1.0;
    for _ in 0..(c as usize) {
        e_admitted_alla_c *= e_admitted;
    }
    let mut result = e_admitted_alla_c * e_resto;

    if is_negative {
        result = 1.0 / result;
    }
    result
}

pub fn traslatore_dominio_seni(x: f64) -> f64 {
    let entita_di_molteplicita = x / (2.0 * PI);
    let parte_sottomultipla = entita_di_molteplicita - (entita_di_molteplicita as i64) as f64;
    (2.0 * PI) * parte_sottomultipla
}

pub fn sin_f64(x: f64) -> f64 {
    let x_trans = traslatore_dominio_seni(x);
    let mut res = 0.0;
    let mut the_fact = 1.0;
    for k in 0..29 {
        if k > 1 {
            the_fact *= k as f64;
        }
        let sign = if is_odd(k) { -1.0 } else { 1.0 };
        res += sign / the_fact * nat_pow(x_trans, 2 * k + 1);
    }
    res
}

pub fn cos_f64(x: f64) -> f64 {
    let x_trans = traslatore_dominio_seni(x);
    let mut res = 0.0;
    let mut the_fact = 1.0;
    for k in 0..29 {
        if k > 1 {
            the_fact *= k as f64;
        }
        let sign = if is_odd(k) { -1.0 } else { 1.0 };
        res += sign / the_fact * nat_pow(x_trans, 2 * k);
    }
    res
}

pub fn tan_f64(x: f64) -> Result<f64, Domain> {
    let denominator = cos_f64(x);
    if fabs(denominator) < 1e-60 {
        return Err(1);
    }
    Ok(sin_f64(x) / denominator)
}


pub fn arctan(y: f64) -> f64 {
    let mut val = y;
    let mut is_negative = false;
    if val < 0.0 {
        val *= -1.0;
        is_negative = true;
    }
    let x;
    let mut is_transformed = false;
    if val > 1.0 {
        x = 1.0 / val;
        is_transformed = true;
    } else {
        x = val;
    }
    let the_distance = fabs(1.0 - x);
    let mut res = 0.0;
    if fabs(the_distance) > 1e-7 {
        let order = 26 * (1.0 / the_distance) as usize;
        let x_sq = x * x;         // x^2 term
        let mut x_pow = x;        // Tracks x^(2k+1), starting at x^1 (k=0)

        for k in 0..order {
            let sign = if is_odd(k) { -1.0 } else { 1.0 };
            res += sign / (2.0 * (k as f64) + 1.0) * x_pow;
            x_pow *= x_sq;        // Incrementally advance to the next odd power
        }
    } else {
        res = PI / 4.0;
    }

    if is_transformed {
        res = PI / 2.0 - res;
    }
    if is_negative {
        res *= -1.0;
    }
    res
}  

pub fn sinh(x: f64) -> f64 {
    let mut res = 0.0;
    let mut the_fact = 1.0;
    for k in 0..70 {
        if k > 1 {
            the_fact *= k as f64;
        }
        res += 1.0 / the_fact * nat_pow(x, 2 * k + 1);
    }
    res
}

pub fn cosh(x: f64) -> f64 {
    let mut res = 0.0;
    let mut the_fact = 1.0;
    for k in 0..70 {
        if k > 1 {
            the_fact *= k as f64;
        }
        res += 1.0 / the_fact * nat_pow(x, 2 * k);
    }
    res
}

pub fn tanh(x: f64) -> f64 {
    sinh(x) / cosh(x)
}

pub mod two_points_interpolation {
    use super::*;

    #[derive(Debug, Clone, Copy)]
    pub struct LinearVarietyCoefficients {
        pub alpha: f64,
        pub beta: f64,
        pub is_problem_well_posed: bool,
    }

    #[derive(Debug, Clone, Copy)]
    pub struct ParametricLinearManifold {
        pub alpha_x: f64,
        pub beta_x: f64,
        pub alpha_y: f64,
        pub beta_y: f64,
        pub is_problem_well_posed: bool,
    }

    #[derive(Debug, Clone, Copy)]
    pub struct ExponentialVarietyCoefficients {
        pub gamma: f64,
        pub chi: f64,
        pub psi: f64,
        pub fi: f64,
    }

    pub fn linear_image(coefficients: LinearVarietyCoefficients, x: f64) -> f64 {
        coefficients.alpha * x + coefficients.beta
    }

    pub fn linear_ante_image(coefficients: LinearVarietyCoefficients, y: f64) -> f64 {
        let c = 1.0 / coefficients.alpha;
        let d = -coefficients.beta / coefficients.alpha;
        c * y + d
    }

    pub fn linear(left: Couple, right: Couple) -> LinearVarietyCoefficients {
        let x0 = left.argument;
        let x1 = right.argument;
        let y0 = left.image;
        let y1 = right.image;

        if fabs(x1 - x0) < 1.0e-12 {
            return LinearVarietyCoefficients {
                alpha: f64::NAN,
                beta: f64::NAN,
                is_problem_well_posed: false,
            };
        }
        let alpha = (y1 - y0) / (x1 - x0);
        let beta = y0 - x0 * alpha;
        LinearVarietyCoefficients {
            alpha,
            beta,
            is_problem_well_posed: true,
        }
    }

    pub fn linear_parametric(left: Couple, right: Couple) -> ParametricLinearManifold {
        let x0 = left.argument;
        let x1 = right.argument;
        let y0 = left.image;
        let y1 = right.image;

        if fabs(x1 - x0) < 1.0e-12 && fabs(y1 - y0) < 1.0e-12 {
            return ParametricLinearManifold {
                alpha_x: f64::NAN,
                beta_x: f64::NAN,
                alpha_y: f64::NAN,
                beta_y: f64::NAN,
                is_problem_well_posed: false,
            };
        }
        if fabs(x1 - x0) < 1.0e-12 {
            return ParametricLinearManifold {
                alpha_x: 0.0,
                beta_x: x0,
                alpha_y: 1.0,
                beta_y: 0.0,
                is_problem_well_posed: true,
            };
        }
        ParametricLinearManifold {
            alpha_x: 1.0,
            beta_x: 0.0,
            alpha_y: (y1 - y0) / (x1 - x0),
            beta_y: y0 - x0 * ((y1 - y0) / (x1 - x0)),
            is_problem_well_posed: true,
        }
    }

    pub fn parametric_linear_image(
        parametric_layout: ParametricLinearManifold,
        independent_parameter: f64,
    ) -> Option<[f64; 2]> {
        if !parametric_layout.is_problem_well_posed {
            return None;
        }
        let x = parametric_layout.alpha_x * independent_parameter + parametric_layout.beta_x;
        let y = parametric_layout.alpha_y * independent_parameter + parametric_layout.beta_y;
        Some([x, y])
    }

    pub fn parametric_linear_ante_image(
        parametric_layout: ParametricLinearManifold,
        x0: f64,
        y0: f64,
        x1: f64,
        y1: f64,
    ) -> Option<[f64; 2]> {
        if !parametric_layout.is_problem_well_posed {
            return None;
        }
        let mut mat = RealMatrix::new(2, 2);
        let _ = mat.insert(parametric_layout.alpha_x, 0, 0);
        let _ = mat.insert(parametric_layout.beta_x, 0, 1);
        let _ = mat.insert(parametric_layout.alpha_y, 1, 0);
        let _ = mat.insert(parametric_layout.beta_y, 1, 1);

        let (pullback_alpha, pullback_beta) = match (|| {
            mat.show(4);
            if fabs(mat.get_at(0, 0).unwrap_or(1.0)) < 1e-20
                && fabs(mat.get_at(1, 1).unwrap_or(1.0)) < 1e-20
            {
                Ok((y0, y1))
            } else if fabs(mat.get_at(0, 0).unwrap_or(0.0) - 1.0) < 1e-20 {
                Ok((x0, x1))
            } else {
                Err(Crash::new("DEBUG: unknown matrix layout."))
            }
        })() {
            Ok(pair) => pair,
            Err(_) => {
                println!("\n\t from inside: catch( Crash curExcp)");
                (x0, x1)
            }
        };

        Some([pullback_alpha, pullback_beta])
    }

    pub fn log_linear_image(coefficients: ExponentialVarietyCoefficients, x: f64) -> Result<f64, Domain> {
        let inner = (x - coefficients.psi) / coefficients.fi;
        let pow_val = pow_f64(coefficients.chi, inner)?;
        Ok(coefficients.gamma * pow_val)
    }

    pub fn log_linear_ante_image(coefficients: ExponentialVarietyCoefficients, y: f64) -> Result<f64, Domain> {
        let log_num = ln(y / coefficients.gamma)?;
        let log_den = ln(coefficients.chi)?;
        let log_base_chi = log_num / log_den;
        Ok(coefficients.fi * log_base_chi + coefficients.psi)
    }

    pub fn log_linear(left: Couple, right: Couple) -> Result<ExponentialVarietyCoefficients, Domain> {
        let x0 = left.argument;
        let x1 = right.argument;
        let y0 = left.image;
        let y1 = right.image;

        if fabs(x1 - x0) < 1.0e-12 {
            return Err(1);
        }
        if fabs(left.image) < 1.0e-12 {
            return Err(1);
        }

        Ok(ExponentialVarietyCoefficients {
            gamma: y0,
            chi: y1 / y0,
            psi: x0,
            fi: x1 - x0,
        })
    }
}

pub mod newton_raphson {
    use super::*;

    pub type Fp = fn(f64) -> f64;

    pub fn f1(x: f64, f: Fp, dx: f64) -> f64 {
        (f(x + dx) - f(x)) / dx
    }

    pub fn nr(xn: f64, f: Fp, dx: f64) -> f64 {
        xn - f(xn) / f1(xn, f, dx)
    }

    pub fn newton_raphson(x0: f64, f: Fp, dx: f64, max_steps: u16, epsilon: f64) -> f64 {
        let mut steps = 0;
        let mut x = x0;
        while fabs(f(x)) > epsilon && steps < max_steps {
            x = nr(x, f, dx);
            steps += 1;
        }
        x
    }
}